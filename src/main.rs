use std::time::Duration;

use opencv::{highgui, imgcodecs, prelude::*};

fn main() {
    let img = imgcodecs::imread("frieren7.jpg", imgcodecs::IMREAD_COLOR).unwrap();
    highgui::imshow("window", &img).unwrap();
    highgui::wait_key(0).unwrap();
}
