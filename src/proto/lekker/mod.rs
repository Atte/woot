pub mod feature_report;
// pub mod raw_report;

pub use feature_report::*;
// pub use raw_report::*;

pub const DEVICE_IDS: &[u16] = &[
    4624, // Wooting Two Lekker Edition
    4640, // Wooting Two HE
    4864, // Wooting 60HE
];

pub const RESPONSE_SIZE: usize = 256;
pub const FEATURE_REPORT_REQUEST_SIZE: usize = 8;
pub const RAW_REPORT_REQUEST_SIZE: usize = 257;
