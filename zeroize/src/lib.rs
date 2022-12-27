use zeroize::Zeroize;

#[derive(Debug)]
struct EncryptionKeyWrong([u8; 4]);

impl EncryptionKeyWrong {
    fn new() -> Self {
        EncryptionKeyWrong(*b"akey")
    }
}

impl Drop for EncryptionKeyWrong {
    fn drop(&mut self) {
        self.0.zeroize()
    }
}

#[derive(Debug)]
struct EncryptionKeyRight(Box<[u8; 4]>);

impl EncryptionKeyRight {
    fn new() -> Self {
        EncryptionKeyRight(Box::new(*b"akey"))
    }
}

impl Drop for EncryptionKeyRight {
    fn drop(&mut self) {
        self.0.zeroize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrong() {
        let my_key = EncryptionKeyWrong::new();
        let ptr = my_key.0.as_ptr();

        println!("Data is currently at address {:?}", ptr);
        drop(my_key);
        println!("Key dropped");

        println!("Memory at pointer location {:?}", unsafe {
            core::slice::from_raw_parts(ptr, 4)
        });
    }

    #[test]
    fn test_right() {
        let my_key = EncryptionKeyRight::new();
        let ptr = my_key.0.as_ptr();

        println!("Data is currently at address {:?}", ptr);
        drop(my_key);
        println!("Key dropped");

        println!("Memory at pointer location {:?}", unsafe {
            core::slice::from_raw_parts(ptr, 4)
        });
    }
}
