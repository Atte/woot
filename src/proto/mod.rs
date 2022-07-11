use crate::Result;
use bytes::{Buf, BufMut};

pub mod lekker;

pub const VENDOR_ID: u16 = 12771;
pub const USAGE_PAGE: u16 = 4919;

pub trait Report {
    const TYPE: u8;
    type Response;

    #[allow(unused_variables)]
    fn write_request(&self, buffer: &mut impl BufMut) {}
    fn read_response(buffer: impl Buf) -> Result<Self::Response>;
}
