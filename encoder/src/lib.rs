pub mod encryption; 

struct KeyStream<'a> {
    key: &'a [u8],
    cursor: usize
}

impl<'a> KeyStream<'a> {
    fn new(key: &'a [u8]) -> Self {
        KeyStream { key, cursor: 0 }
    }
}

impl<'a> Iterator for KeyStream<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.key.len() {
            self.cursor = 0;
        }
        let next: Option<&u8> = self.key.get(self.cursor);
        self.cursor += 1;
        next.copied()
    }
}



#[cfg(test)]
mod tests {
    use crate::KeyStream;

    #[test]
    fn test_infinit_iterato() {
        let mut keystream = KeyStream::new(&[1,2,3,4,5]);
        keystream.nth(10000).unwrap();
    } 
}
