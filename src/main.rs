struct Coordinate{
    x:u32,
    y:u32
}

struct ImageSetting {
    pointer_coordinate:Coordinate,
    pointer_byte: u64,
    pixcel_size:u32,
    width:u32,
    height:u32,
    name:u64,
    image:image::ImageBuffer<image::Rgb<u8>, Vec<u8>>
}

struct FileHeader {
    file_name:String,
    file_size:u64,
}



const  OUTPUT_DIR: &str = "./out/";
const  FILE_TYPE:&str=".bmp";
const WIDTH:u32 = 1920;
const HEIGHT:u32 = 1080;
const BYTE:u8 = 8;


mod definition;

mod conversion;
mod restoration;
// 

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("mode: convert or deconvert");
    let mode = get_input();

    if mode == "convert"{
        println!("input_file:./");
        let input_fl = get_input();
        

        println!("output_move:./");
        let output_mv = get_input();

        println!("framerate:u32 :");
        let framerate:u32 = get_input().parse().unwrap();

        println!("pixcel_size:u32 :");
        let pixcel_size:u32 = get_input().parse().unwrap();
        
        convert(input_fl, output_mv, framerate,pixcel_size);
        println!("convert successful")

    }
    else {
        
        println!("input_move:./");
        let input_mv = get_input();

        println!("pixcel_size:u32 :");
        let pixcel_size:u32 = get_input().parse().unwrap();
        

        deconvert(input_mv,pixcel_size);
        println!("deconvert successful");

    }


    Ok(())
}

fn convert(input_fl:String,output_mv:String,framerate:u32,pixcel_size:u32){
    definition::clear();
    conversion::make_image(&input_fl,pixcel_size);
    conversion::make_movie(framerate,&output_mv);
    // definition::clear();
}

#[test]
fn test_con(){
    convert("test.txt".to_string(), "test.mp4".to_string(), 60, 2);
}


fn deconvert(input_mv:String,pixcel_size:u32){
    definition::clear();
    restoration::cut_out_image(&input_mv);
    restoration::make_file(pixcel_size);
    definition::clear();
}

#[test]
fn test_decon(){
    deconvert("test.mp4".to_string(),2,);
}



fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}