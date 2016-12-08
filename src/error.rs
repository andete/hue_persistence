// (c) 2016 Joost Yervante Damad <joost@damad.be>

use philipshue::errors::HueError;
use ssdp::SSDPError;
use std::io;

#[derive(Debug)]
pub enum Error {
    HueError(HueError),
    SSDPError(SSDPError),
    IOError(io::Error),
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

impl From<io::Error> for Error {
    fn from(e:io::Error) -> Error {
        Error::IOError(e)
    }
}
