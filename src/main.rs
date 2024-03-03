use std::time;

struct Coordinate {
    x: u32,
    y: u32,
}

struct ImageSetting {
    pointer_coordinate: Coordinate,
    pixcel_size: u32,
    width: u32,
    height: u32,
    name: u64,
    image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

struct FileHeader {
    file_name: String,
    file_size: u64,
}

const OUTPUT_DIR: &str = "./out/";
const FILE_TYPE: &str = "png";
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const BYTE: u32 = 8;

mod definition;

mod conversion;
mod restoration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("mode: convert or deconvert");
    let mode = get_input();

    if mode == "convert" {
        println!("input_file:./");
        let input_fl = get_input();

        println!("output_move:./");
        let output_mv = get_input();

        println!("framerate:u32 :");
        let framerate: u32 = get_input().parse().unwrap();

        println!("pixcel_size:u32 :");
        let pixcel_size: u32 = get_input().parse().unwrap();

        convert(input_fl, output_mv, framerate, pixcel_size);
        println!("convert successful")
    } else {
        println!("input_move:./");
        let input_mv = get_input();

        println!("pixcel_size:u32 :");
        let pixcel_size: u32 = get_input().parse().unwrap();

        deconvert(input_mv, pixcel_size);
        println!("deconvert successful");
    }

    Ok(())
}

fn convert(input_fl: String, output_mv: String, framerate: u32, pixcel_size: u32) {
    definition::clear();

    let now = time::Instant::now();
    conversion::make_img_mv(&input_fl, pixcel_size, framerate, &output_mv);
    println!("{:?}", now.elapsed());

    definition::clear();
}

fn deconvert(input_mv: String, pixcel_size: u32) {
    definition::clear();
    let now = time::Instant::now();
    restoration::cut_out_image(&input_mv);
    restoration::make_file(pixcel_size);
    println!("{:?}", now.elapsed());

    definition::clear();
}

fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}
