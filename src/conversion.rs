use image;
use std;
use std::io::Read;
use std::io::Write;
use std::process::ChildStdin;
#[path ="./definition.rs"]

mod definition;


use crate::{
    WIDTH,
    HEIGHT,
    ImageSetting,
    Coordinate,
    FileHeader,
};


fn make_movie(mut child_stdin:& ChildStdin,img_set:&ImageSetting) {

    let raw_pixels = img_set.image.clone().into_raw();

    child_stdin.write_all(&raw_pixels).expect("Failed to write to stdin");

}


pub fn make_img_mv(input_fl:&str,pixcel_size:u32,framerate: u32,output_mv:&str){
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
        image:image::RgbImage::new(WIDTH, HEIGHT)
    };
    let file = std::fs::File::open(input_fl).expect("no such file or directory");
    let file_header = FileHeader{
        file_name:input_fl.to_string(),
        file_size:file.metadata().unwrap().len()
    };
    let mut reader = std::io::BufReader::new(file);
    let mut buf = [0; std::u8::MAX as usize];

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
    
    let child_stdin = child.stdin.as_mut().expect("failed to get stdin");
    

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
                            
                            make_movie(&child_stdin, &img_set);


                            img_set.image =image::RgbImage::new(WIDTH, HEIGHT);
                            img_set.name+=1;
                            img_set.pointer_coordinate.x=0;
                            img_set.pointer_coordinate.y=0;
                        }
                    } 
                }

                
            }
        }
    }

    make_movie(&child_stdin, &img_set);

    let _ = child.wait().expect("child process wasn't running");
}


fn write_pixel(img_set:& mut ImageSetting,bit:bool){
    for x in 0..img_set.pixcel_size {
        
        for y in 0..img_set.pixcel_size{
            
            img_set.image.put_pixel(
                img_set.pointer_coordinate.x+x, 
                img_set.pointer_coordinate.y+y, 
            if bit {image::Rgb([0,0,0])} else{image::Rgb([255,255,255])}
            );
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