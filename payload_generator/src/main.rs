use std::arch::asm;
use encoder::encryption::Encrypt;

fn main() {
    println!("Hello, world!");
    unsafe {
        let payload_bytes = (payload as fn()).encrypt(&[1,2,3,4,5]);
        std::fs::write("../payload.bin", payload_bytes);
    }
}

#[no_mangle]
pub fn payload() {
    unsafe {
        asm! {
            "mov rax, 60",
            "mov rdi, 0",
            "syscall"
        };   
    }
}
