use opencv::{highgui, imgcodecs};

fn main() {
    let img = imgcodecs::imread("1.png", imgcodecs::IMREAD_COLOR).unwrap();
    highgui::imshow("window", &img).unwrap();
    highgui::start_window_thread().unwrap();
    highgui::wait_key(0).unwrap();
}
