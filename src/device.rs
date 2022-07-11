use crate::{
    proto::{
        lekker::{FEATURE_REPORT_REQUEST_SIZE, RESPONSE_SIZE},
        Report,
    },
    Result,
};
use bytes::{Buf, BytesMut};
use hidapi::HidDevice;

pub struct Device {
    device: HidDevice,
}

impl Device {
    #[must_use]
    #[inline]
    pub fn new(device: HidDevice) -> Self {
        Self { device }
    }

    pub fn feature_report<T: Report>(&self, fr: T) -> Result<T::Response> {
        let mut buffer = BytesMut::with_capacity(FEATURE_REPORT_REQUEST_SIZE);
        buffer.extend_from_slice(&[0, 208, 218, T::TYPE]);
        fr.write_request(&mut buffer);
        buffer.resize(FEATURE_REPORT_REQUEST_SIZE, 0);
        self.device.send_feature_report(&buffer)?;

        let mut buffer = BytesMut::with_capacity(RESPONSE_SIZE);
        buffer.resize(RESPONSE_SIZE, 0);
        self.device.read(&mut buffer)?;

        assert_eq!(buffer.get_u8(), 208);
        assert_eq!(buffer.get_u8(), 218);
        assert_eq!(buffer.get_u8(), T::TYPE);
        assert_eq!(buffer.get_u8(), 136);

        let len = buffer.get_u8() as usize;
        buffer.resize(len, 0);

        T::read_response(buffer)
    }
}
