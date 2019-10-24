use super::buffer::MBuf;
use super::decoder::Decoder;
use tokio::io::AsyncRead;

pub struct FrameReader<R: AsyncRead, D: Decoder> {
    reader: R,
    decoder: D,
}

impl<R: AsyncRead, D: Decoder> FrameReader<R, D> {
    pub fn new(reader: R, decoder: D) -> Self {
        FrameReader { reader, decoder }
    }
    pub async fn async_next_frame(&mut self) -> Result<Option<D::Item>, D::Error> {
        let mut buffer = MBuf::new(22);
        self.decoder.decode(&mut buffer)
    }
}
