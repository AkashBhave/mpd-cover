extern crate mpd;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use mpd::Client;

pub fn main() {
    let mut stream = UnixStream::connect("/home/akash/.config/mpd/socket").unwrap();
    let mut c = Client::new(stream).unwrap();
    println!("{}", c.music_directory().unwrap());
}
