use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};

pub fn parser() -> ArgMatches {
    let matches = Command::new("LocalRpiCam")
        .version("0.1")
        .about("Hopefully streams RPi Camera to Webserver")
        .arg(arg!(--device <VALUE>).required(true).short('d'))
        .arg(arg!(--ipv4 <VALUE>).required(true).short('i'))
        .arg(arg!(--port <VALUE>).required(true).short('p'))
        .get_matches();

    return matches;
}
