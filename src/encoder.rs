use super::buffer;

pub trait EnCoder {
    type Item;
    type Error: std::error::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut buffer::MBuf) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::super::error::*;
    use super::*;
    use std::io::Read;

    use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
    use std::mem;

    #[derive(Default)]
    struct TestEncoder {}

    impl EnCoder for TestEncoder {
        type Item = i64;
        type Error = Error;

        fn encode(&mut self, item: Self::Item, buf: &mut buffer::MBuf) -> Result<(), Self::Error> {
            let mut pbuf = buf.prepare(mem::size_of::<i64>())?;
            pbuf.write_i64::<LittleEndian>(item)?;
            buf.commit(mem::size_of::<i64>())?;
            Ok(())
        }
    }

    #[test]
    fn test_encoder() {
        let mut encoder = TestEncoder::default();
        let mut buf = buffer::MBuf::new(1024);
        if let Err(e) = encoder.encode(3i64, &mut buf) {
            println!("{:?}", e.description());
        }
        let data = buf.data();
        assert_eq!(3i64, LittleEndian::read_i64(data));
    }
}
