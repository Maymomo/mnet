use super::buffer;

pub trait Decoder {
    type Item;
    type Error: std::error::Error;

    fn decode(&mut self, buf: &mut buffer::MBuf) -> Result<Option<Self::Item>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::super::error::*;
    use super::*;
    use std::io::Read;

    use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
    use std::mem;

    #[derive(Default)]
    struct TestDecoder {
        count: i32,
    }

    impl Decoder for TestDecoder {
        type Item = i64;
        type Error = Error;

        fn decode(&mut self, buf: &mut buffer::MBuf) -> Result<Option<Self::Item>, Self::Error> {
            if self.count < 1 {
                self.count += 1;
                return Ok(None);
            }
            return Ok(Some(1));
        }
    }

    #[test]
    fn test_decode() {
        let mut decoder = TestDecoder::default();
        let mut buf = buffer::MBuf::new(1024);
        assert_eq!(None, decoder.decode(&mut buf).unwrap());
        assert_eq!(Some(1), decoder.decode(&mut buf).unwrap());
    }
}
