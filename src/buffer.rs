#[derive(Debug)]
pub enum MBufErr {
    NOMORESIZE,
    CONSUMEERR,
    COMMITERR,
}

impl std::fmt::Display for MBufErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for MBufErr {
    fn description(&self) -> &str {
        match *self {
            MBufErr::NOMORESIZE => {
                &"buffer has no more size"
            },
            MBufErr::CONSUMEERR => {
                &"buffer consume error"
            },
            MBufErr::COMMITERR => {
                &"buffer commit error"
            }
        }
    }
}

pub struct MBuf {
    buf: Vec<u8>,
    read: usize,
    write: usize,
    cap: usize,
}

impl MBuf {
    pub fn new(buf_cap: usize) -> MBuf {
        MBuf {
            buf: vec![0; buf_cap],
            read: 0,
            write: 0,
            cap: buf_cap,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.buf.as_slice()[self.read..self.write]
    }

    pub fn consume(&mut self, size: usize) -> Result<(), MBufErr> {
        let new_read = self.read + size;
        if new_read > self.write {
            return Err(MBufErr::CONSUMEERR);
        }
        self.read = new_read;
        Ok(())
    }

    pub fn prepare(&mut self, size: usize) -> Result<&mut [u8], MBufErr> {
        self.reserve(size)?;
        Ok(&mut self.buf.as_mut_slice()[self.write..self.write + size])
    }

    pub fn commit(&mut self, size: usize) -> Result<(), MBufErr> {
        let new_write = self.write + size;
        if new_write > self.cap {
            return Err(MBufErr::COMMITERR);
        }
        self.write = new_write;
        Ok(())
    }

    fn reserve(&mut self, size: usize) -> Result<(), MBufErr> {
        if self.cap() - self.write >= size {
            return Ok(());
        }

        if self.remain() < size {
            return Err(MBufErr::NOMORESIZE);
        }

        unsafe {
            let ptr = self.buf.as_mut_ptr();
            std::ptr::copy(ptr.add(self.read), ptr, self.len());
        }

        let old_len = self.len();
        self.read = 0;
        self.write = old_len;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.write - self.read
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remain(&self) -> usize {
        self.cap() - self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_buffer() {
        let mut buffer = MBuf::new(1024);
        let nread = read_data(buffer.prepare(10).unwrap(), 5, 1);
        buffer.commit(nread).unwrap();
        assert_eq!(5, buffer.len());
        assert_eq!(1024 - 5, buffer.remain());
        for i in buffer.data() {
            assert_eq!(1 as u8, *i);
        }
        buffer.consume(5).unwrap();
        assert_eq!(0, buffer.len());
        assert_eq!(buffer.cap(), buffer.remain());
        let nread = read_data(buffer.prepare(1024).unwrap(), 1024, 2);
        buffer.commit(nread).unwrap();
        assert_eq!(0, buffer.remain());
        assert_eq!(buffer.cap(), buffer.len());
        for i in buffer.data() {
            assert_eq!(2 as u8, *i);
        }

        match buffer.commit(10000) {
            Err(_) => {}
            _ => {
                panic!("commit error");
            }
        }
        match buffer.consume(10000) {
            Err(_) => {}
            _ => {
                panic!("consume error");
            }
        }
    }

    fn read_data(buf: &mut [u8], nread: usize, v: u8) -> usize {
        for i in 0..nread {
            buf[i as usize] = v;
        }
        nread
    }
}
