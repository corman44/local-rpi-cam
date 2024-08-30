extern crate ctrlc;
extern crate gstreamer as gst;
use local_rpi_cam::parser::parser;
use std::{env, process};
use gst::prelude::*;

fn usage(args: Vec<String>) {
    println!("Usage: {} port", args[0]);
}

fn main() {
    // Read command line arguments
    let matches = parser();

    let device = matches.get_one::<String>("device").expect("required");
    let ipv4 = matches.get_one::<String>("ipv4").expect("required");
    let port = matches.get_one::<String>("port").expect("required");
    // start_stream(&device, &ipv4, &port);

    // println!("Your Input:");
    // println!("Device: {}", matches.get_one::<String>("device").expect("required"));
    // println!("IPv4: {}", matches.get_one::<String>("ipv4").expect("required"));
    // println!("Port: {}", matches.get_one::<String>("port").expect("required"));
}