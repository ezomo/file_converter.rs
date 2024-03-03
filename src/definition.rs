use std;

use crate::OUTPUT_DIR;

#[allow(dead_code)]
pub fn bool_array_to_u8(array: [bool; 8]) -> u8 {
    let mut re_u8: u8 = 0;

    for i in 0..array.len() {
        re_u8 += 2_u8.pow(i as u32) * array[array.len() - i - 1] as u8;
    }

    re_u8
}

#[allow(dead_code)]
pub fn u8_to_bool_array(n: u8) -> [bool; 8] {
    let mut arr = [false; 8];
    for i in 0..8 {
        arr[i] = ((n >> i) & 1) == 1;
    }
    arr.reverse();
    arr
}

#[allow(dead_code)]
pub fn clear() {
    if std::path::Path::new(OUTPUT_DIR).is_dir() {
        // ディレクトリが存在するなら丸ごと削除
        let _ = std::fs::remove_dir_all(OUTPUT_DIR);
    }
    let _ = std::fs::create_dir(OUTPUT_DIR);
}

#[test]
fn test_clear() {
    clear()
}

#[allow(dead_code)]
pub fn frame_number() -> Result<usize, Box<dyn std::error::Error>> {
    let paths = std::fs::read_dir(OUTPUT_DIR)?;
    let frame_count = paths.filter_map(Result::ok).count();
    Ok(frame_count)
}
