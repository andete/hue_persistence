// (c) 2016 Joost Yervante Damad <joost@damad.be>

use philips_hue_client::errors::HueError;
use ssdp::SSDPError;

#[derive(Debug)]
pub enum Error {
    HueError(HueError),
    SSDPError(SSDPError),
}

impl From<SSDPError> for Error {
    fn from(e:SSDPError) -> Error {
        Error::SSDPError(e)
    }
}

impl From<HueError> for Error {
    fn from(e:HueError) -> Error {
        Error::HueError(e)
    }
}
