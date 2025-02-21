use std::{io::ErrorKind, ptr, mem};
use libc::{mmap, PROT_WRITE, MAP_PRIVATE, MAP_ANON};
use crate::KeyStream;

pub trait Encrypt {
    type Out;
    fn encrypt(&self, key: &[u8]) -> Self::Out;
}

pub trait Decrypt<T> {
    fn decrypt(&self, key: &[u8]) -> Result<T, std::io::Error>;
}


impl Encrypt for fn() {
    fn encrypt(&self, key: &[u8]) -> Self::Out{
        let mut keystream = KeyStream::new(key);
        let mut bytes = Vec::new();
        unsafe {
            let mut fn_ptr: *const u8 = (*self) as *const u8;
            let mut current_byte = *fn_ptr;
            while current_byte != 0xc3 {
                println!("current: {current_byte}");
                bytes.push(current_byte ^ keystream.next().unwrap());
                fn_ptr = fn_ptr.add(1);
                current_byte = *(fn_ptr);
            }
            bytes.push(0xc3 ^ keystream.next().unwrap());
        }
        bytes
    }

    type Out = Vec<u8>;
}

impl<T: AsRef<[u8]>> Decrypt<fn()> for T {
    fn decrypt(&self, key: &[u8]) -> Result<fn(), std::io::Error> {
        let mut keystream = KeyStream::new(key);
        let closure = |(p, k): (&u8, u8)| {
            *p ^ k
        };
        let bytes = self.as_ref().iter().zip(keystream).map(closure).collect::<Vec<u8>>();
        unsafe {
            let map = mmap(ptr::null_mut(), bytes.len(), libc::PROT_EXEC | libc::PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, 0,0);
            if map.is_null() {
                return Err(std::io::Error::from(ErrorKind::Other));
            }
            ptr::copy(bytes.as_ptr(), map as *mut u8, bytes.len());
            let func: fn() = mem::transmute(map);
            return Ok(func);
        }
    }
}

impl Encrypt for String {
    type Out = Vec<u8>;

    fn encrypt(&self, key: &[u8]) -> Self::Out {
        let mut keystream = KeyStream::new(key);
        
        self.bytes().map(|byte| {
            byte ^ keystream.next().unwrap()
        }).collect::<Vec<u8>>()
    }
}

impl Decrypt<String> for Vec<u8> {
    fn decrypt(&self, key: &[u8]) -> Result<String, std::io::Error> {
        let mut keystream = KeyStream::new(key);
        
        let decrypted = self.into_iter().map(|byte| {
            byte ^ keystream.next().unwrap()
        }).collect::<Vec<u8>>();
        
        String::from_utf8(decrypted).map_err(|err| {
            eprintln!("{err}");
            std::io::Error::from(ErrorKind::InvalidInput)
        })
    }
}
