pub mod one_time_pad {
    use rand::{thread_rng, Rng};

    #[derive(Debug, Clone)]
    pub struct OneTimePad {
        original: String,
        padded: String,
    }

    impl OneTimePad {
        pub fn new(original: String) -> OneTimePad {
            return OneTimePad {
                original,
                padded: String::new(),
            };
        }

        /// Returns the key.
        pub fn encode_gen_key(&mut self) -> Vec<u8> {
            let mut key: Vec<u8> = Vec::new();
            for src_byte in self.original.bytes() {
                let key_byte: u8 = thread_rng().gen();
                key.push(key_byte);
                self.padded.push((src_byte ^ key_byte) as char);
            }

            key
        }

        /// Returns the key.
        pub fn encode(&mut self, key: Vec<u8>) -> () {
            for (i, src_byte) in self.original.chars().enumerate() {
                self.padded.push((src_byte as u8 ^ key[i]) as char);
            }
        }

        pub fn decode(&self, key: Vec<u8>) -> String {
            let mut dec = String::new();
            for (i, src_byte) in self.padded.chars().enumerate() {
                dec.push((src_byte as u8 ^ key[i]) as char);
            }

            dec
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode() {
        use crate::one_time_pad::OneTimePad;
        let hw = "Hello world!";
        let mut otp = OneTimePad::new(String::from(hw));

        let key = otp.encode_gen_key();

        assert_eq!(otp.decode(key), hw);
    }

    #[test]
    fn decode_with_key() {
        use crate::one_time_pad::OneTimePad;

        let key: Vec<u8> = [70, 213, 250, 164, 65, 186, 239, 111, 50, 37, 62, 133].to_vec();

        let hw = "Hello world!";
        let mut otp = OneTimePad::new(String::from(hw));

        otp.encode(key.clone());

        assert_eq!(otp.decode(key), hw);
    }
}
