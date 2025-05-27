fn main() {
    if cfg!(windows) {
        let opencv_dir = "D:/personal/opencv-test/opencv/build";

        println!("cargo:rustc-link-lib=opencv_world4110d");
        println!("cargo:rustc-link-search=native={opencv_dir}/x64/vc16/lib");
        println!("cargo:rustc-env=PATH={opencv_dir}/x64/vc16/bin");
    }
}
