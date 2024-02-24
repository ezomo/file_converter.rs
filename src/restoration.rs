use std::io::Write;



#[path ="./definition.rs"]
mod definition;

use crate::{
    Coordinate, FileHeader, ImageSetting, BYTE, FILE_TYPE, OUTPUT_DIR
};


pub fn make_file(pixcel_size:u32){

    let mut img = bmp::open(format!("{}{}{}",OUTPUT_DIR,1,FILE_TYPE).as_str()).expect("No such file or directory");
    let mut img_set = ImageSetting{
        pointer_coordinate: Coordinate{
            x: 0,
            y: 0
        },
        pointer_byte:0, //仮おき
        pixcel_size:pixcel_size,
        width: img.get_width(),
        height: img.get_height(),
        name:1,
        image:img
    };
    let mut  file_data:Vec<u8> = vec![];

    {
        let file_size = (img_set.width*img_set.height/img_set.pixcel_size.pow(2))as u64;

        read_file(&mut img_set, &mut file_data,&{FileHeader{
            file_name:"".to_string(),
            file_size:file_size
        }});    
    }
    

    
    let file_header = get_header(& mut file_data);
    img_set.pointer_byte -= file_header.file_name.len() as u64 + 2 +(u64::BITS/BYTE as u32) as u64;




    let mut writer = std::io::BufWriter::new(std::fs::File::create(&file_header.file_name).expect("IDK"));
    let _ = writer.write_all(&file_data);

    file_data.clear();

    let frame_number = definition::frame_number().expect("No such file or directory");


    for frame in 2..=frame_number{

        img = bmp::open(format!("{}{}{}",OUTPUT_DIR,frame,FILE_TYPE).as_str()).expect("No such file or directory");
        file_data = vec![];


        {
            img_set.image = img;
            img_set.name = frame as u64;
            img_set.pointer_coordinate.x = 0;
            img_set.pointer_coordinate.y = 0;

        }
        
        read_file(&mut img_set, &mut file_data,&file_header);

        let _ = writer.write_all(&file_data);
        file_data.clear();

    }

    let _ = writer.flush();


}


pub fn cut_out_image(input_mv:&str){

    let mut child = std::process::Command::new("/bin/sh")
        .args(&["-c", &{
            format!("ffmpeg -i {} -vcodec bmp {}%d{}",input_mv,OUTPUT_DIR,FILE_TYPE)
        }])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    
    let _child_stdin = child.stdin.as_mut().expect("failed to get stdin");

    let _ = child.wait().expect("child process wasn't running");
}
#[test]
fn test_cut() {
    cut_out_image("rust_book.mp4");   
}

fn read_file(img_set:&mut ImageSetting,file_data:&mut Vec<u8>,file_header:&FileHeader){
    loop {
        let mut byte :[bool;BYTE as usize]= [false;8];
        for bit in 0..BYTE{
            byte[bit as usize] = read_pixel(img_set);
        }
        
        img_set.pointer_byte += 1;
        file_data.push(definition::bool_array_to_u8(byte));

        if img_set.height <= img_set.pointer_coordinate.y || file_header.file_size <= img_set.pointer_byte {
            
            break;
        }
        
    }  
}


fn get_header(file_data:&mut Vec<u8>) ->  FileHeader{


    let mut header = FileHeader{
        file_name:"".to_string(),
        file_size:0
    };

    header.file_name= 
    {
        // get file name
        let mut  file_name:Vec<u8> = vec![];
        for i  in 0..file_data.len(){
            if file_data[i]  == 0{
                break;
            }
            file_name.push(file_data[i]);
        }

        for _ in 0..=file_name.len(){    
            file_data.remove(0);
        }
        String::from_utf8(file_name).unwrap()
    };


    header.file_size = 
    {
        // get file size
        let mut  file_size:[u8;{u64::BITS/BYTE as u32}as usize] = [0;{u64::BITS/BYTE as u32}as usize];

        for i  in 0..file_size.len(){
            file_size[i] = file_data[i];
        }

        for _ in 0..=file_size .len(){    
            file_data.remove(0);
        }

        u64::from_be_bytes(file_size)
    };


    {
        if file_data.len() as u64 > header.file_size{

            for _ in header.file_size..file_data.len() as u64{
                file_data.pop();
            }

        }
    }

    header
}



fn read_pixel(img_set:& mut ImageSetting) -> bool{

    let mut sum:u32 = 0;
    let rgb:u32 = 3;
    let rgb_max:u32 = std::u8::MAX as u32;


    for x in 0..img_set.pixcel_size {
        
        for y in 0..img_set.pixcel_size{
        
            let pixel = img_set.image.get_pixel(img_set.pointer_coordinate.x+x,img_set.pointer_coordinate.y+y);
            sum += (pixel.r as u32 + pixel.g as u32 + pixel.b as u32) / rgb;

        }           
    }

    if (img_set.pointer_coordinate.x+img_set.pixcel_size)/img_set.width > 0{
        img_set.pointer_coordinate.y += img_set.pixcel_size;

    }

    img_set.pointer_coordinate.x = (img_set.pointer_coordinate.x+img_set.pixcel_size)%img_set.width;
    
    
    if sum < (rgb_max/2) as u32 {true } else {false}
}

