extern crate ctrlc;
extern crate gstreamer as gst;
use local_rpi_cam::parser::parser;
use gst::prelude::*;
use std::{env, process};

fn usage(args: Vec<String>) {
    println!("Usage: {} device ipv4 port", args[0]);
}

fn create_pipeline(device: &String, ip: &String, port: &String) -> gst::Pipeline {
    let src = gst::ElementFactory::make_with_name("v4l2src", None)
	.expect("Could not create source element");
    let conv = gst::ElementFactory::make_with_name("videoconvert", None)
	.expect("Could not create source element");
    let enc = gst::ElementFactory::make_with_name("x264enc", None)
        .expect("Could not create x264 encoder");
    let pay = gst::ElementFactory::make_with_name("rtph264pay", None)
        .expect("Could not create RTP payload");
    let sink = gst::ElementFactory::make_with_name("udpsink", None)
        .expect("Could not create UDP sink element");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::new();

    // Build the pipeline
    pipeline.add_many(&[&src, &conv, &enc, &pay, &sink]).unwrap();
    src.link(&conv).expect("Could not link source to video converter");
    conv.link(&enc).expect("Could not link video converter to encoder");
    enc.link(&pay).expect("Could not link encoder to RTP payload");
    pay.link(&sink).expect("Could not link encoder to RTP payload");

    // Direct the source to the specified camera device
    src.set_property_from_str("device", device);

    // Direct the sink to our host
    sink.set_property_from_str("host", ip);
    sink.set_property_from_str("port", port);

    return pipeline;
}

fn start_stream(device: &String, ip: &String, port: &String) {
    // Initialize GStreamer and pipeline;
    println!("Initializing pipeline...");
    gst::init().unwrap();
    let pipeline = create_pipeline(device, ip, port);

    // Gracefully handle a keyboard interrupt (ctrl-C)
    let pipeline_weak = pipeline.downgrade();
    ctrlc::set_handler(move || {
	let pipeline = match pipeline_weak.upgrade() {
	    Some(pipeline) => pipeline,
	    None => return,
	};
	println!("Ending stream. Please wait...");
	// pipeline.send_event(gst::Event::new_eos().build());
    }).expect("Error setting Ctrl-C handler");

    // Start the stream pipeline
    println!("Streaming {} to {}:{}... (Use Ctrl-C to end stream)", device, ip, port);
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),

        }
    }

    pipeline
    .set_state(gst::State::Null)
    .expect("Unable to set the pipeline to the `Null` state");
}


fn main() {
    let matches = parser();
    let device = matches.get_one::<String>("device").expect("required");
    let ipv4 = matches.get_one::<String>("ipv4").expect("required");
    let port = matches.get_one::<String>("port").expect("required");

    start_stream(&device, &ipv4, &port);

    println!("Stream finished");
}