use error_chain::error_chain;

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }
    foreign_links {
        MBufErr(super::buffer::MBufErr);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }
}
