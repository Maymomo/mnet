use super::buffer;
use super::error::*;

pub trait EnCoder {
    type Item;

    fn encode(&mut self, item: Self::Item, buf: &mut buffer::MBuf) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    use byteorder::{LittleEndian, WriteBytesExt};
    use std::mem;

    #[derive(Default)]
    struct TestEncoder {}

    impl EnCoder for TestEncoder {
        type Item = i64;

        fn encode(&mut self, item: Self::Item, buf: &mut buffer::MBuf) -> Result<()> {
            let mut pbuf = buf.prepare(mem::size_of::<i64>())?;
            pbuf.write_i64::<LittleEndian>(item)
                .expect("Unable to write");
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
    }
}
