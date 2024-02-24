use bmp;
use std;
use std::io::Read;
use std::io::Write;
#[path ="./definition.rs"]

mod definition;


use crate::{
    WIDTH,
    HEIGHT,
    OUTPUT_DIR,
    FILE_TYPE,
    ImageSetting,
    Coordinate,
    FileHeader,
};


pub fn make_movie(framerate: u32,output_mv:&str) {

    let mut child = std::process::Command::new("/bin/sh")
        .args(&["-c", &{
            format!(
                "ffmpeg -f rawvideo -pix_fmt rgb24 -s {}x{} -r {} -i - -pix_fmt yuv420p -vcodec libx264 -crf 20 -preset slower -movflags +faststart -y {}",
                WIDTH, HEIGHT, framerate,output_mv
            )
        }])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    {
        let child_stdin = child.stdin.as_mut().expect("failed to get stdin");

        for frame in 1..=definition::frame_number().expect("there is nothing") {
            let filename = format!("{}{}{}",OUTPUT_DIR,frame,FILE_TYPE);
            let img = image::open(&std::path::Path::new(&filename)).unwrap();
            let img_rgb = img.to_rgb8();
            let raw_pixels = img_rgb.into_raw();

            child_stdin.write_all(&raw_pixels).expect("Failed to write to stdin");

        }

    }

    let _ = child.wait().expect("child process wasn't running");
    let _ = definition::clear();

}


pub fn make_image(input_fl:&str,pixcel_size:u32){
    let mut img_set = ImageSetting{
        pointer_coordinate: Coordinate{
            x: 0,
            y: 0
        },
        pointer_byte:0,
        pixcel_size: pixcel_size,
        width: WIDTH,
        height: HEIGHT,
        name:1,
        image: bmp::Image::new(WIDTH, HEIGHT)
    };


    let file = std::fs::File::open(input_fl).expect("no such file or directory");
    let file_header = FileHeader{
        file_name:input_fl.to_string(),
        // 2 for null
        file_size:file.metadata().unwrap().len()
    };
    let mut reader = std::io::BufReader::new(file);
    let mut buf = [0; std::u8::MAX as usize];


    write_header(file_header, &mut img_set);


    loop {
        match reader.read(&mut buf).expect("IDK") {
            0 => break,
            n => {
                let buf = &buf[..n];
                for  byte in buf{
                    for i in definition::u8_to_bool_array(*byte){
                        write_pixel(&mut img_set,i); 

                        if img_set.height <= img_set.pointer_coordinate.y{
                            
                            let _ = img_set.image.save(&(OUTPUT_DIR.to_owned()+&img_set.name.to_string()+FILE_TYPE));
                            
                            img_set.image = bmp::Image::new(img_set.width, img_set.height);
                            img_set.name+=1;
                            img_set.pointer_coordinate.x=0;
                            img_set.pointer_coordinate.y=0;
                        }
                    } 
                }
            }
        }
    }

    let _ = img_set.image.save(&(OUTPUT_DIR.to_owned()+&img_set.name.to_string()+FILE_TYPE));

}


fn write_pixel(img_set:& mut ImageSetting,bit:bool){
    for x in 0..img_set.pixcel_size {
        
        for y in 0..img_set.pixcel_size{
            
            

            img_set.image.set_pixel(
                img_set.pointer_coordinate.x+x,
                img_set.pointer_coordinate.y+y,
                if bit { bmp::Pixel { r: 0, g: 0, b: 0}} 
                    else {bmp::Pixel { r: 255, g: 255, b: 255}})
        }           
    }

    if (img_set.pointer_coordinate.x+img_set.pixcel_size)/img_set.width > 0{
        img_set.pointer_coordinate.y += img_set.pixcel_size;

    }

    img_set.pointer_coordinate.x = (img_set.pointer_coordinate.x+img_set.pixcel_size)%img_set.width;
}


fn write_header(file:FileHeader,img_set:& mut ImageSetting){

    let null:u8 = 0;
    {
        for byte in file.file_name.bytes(){
            for bit in definition::u8_to_bool_array(byte){
                write_pixel(img_set, bit)
            }
        }
    
        for bit in definition::u8_to_bool_array(null){
            write_pixel(img_set, bit)
        }
    }
    

    {
        for byte in file.file_size.to_be_bytes(){
            for bit in definition::u8_to_bool_array(byte){
                write_pixel(img_set, bit)
            }
        }
    
        for bit in definition::u8_to_bool_array(null){
            write_pixel(img_set, bit)
        }

    }
    
    
}