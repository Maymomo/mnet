use error_chain::error_chain;

error_chain! {
    foreign_links {
        MBufErr(super::buffer::MBufErr);
    }
}
