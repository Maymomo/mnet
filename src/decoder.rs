use super::buffer;

pub trait Decoder {
    type Item;
    type Error: std::error::Error;

    fn decode(&mut self, buf: &mut buffer::MBuf) -> Result<Option<Self::Item>, Self::Error>;
}
