# Filer Converter　　
All files can be converted to video files (mp4).  
I made a copycat of [this](https://github.com/DvorakDwarf/Infinite-Storage-Glitch).　　

### dependence  
- [Rust](https://www.rust-lang.org/tools/install) 
- [ffmpeg](https://ffmpeg.org/)

### structure　　
file structure
The file structure is simple.  
*File name* , *file size* (before turning it into a video) , *file*  
Each item is separated by NULL.  

### process Flow
Convert the input file to  image files and save them in ./out/ .  
After that, generate a video using the images stored in ./out/ .　

### Notice
pixel_size is the length of one side.  
Therefore, if the pixel_size is 2, a square of 2*2 represents one bit.
