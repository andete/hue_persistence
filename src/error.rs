// (c) 2016 Joost Yervante Damad <joost@damad.be>

error_chain! {

    links {
        SSDP(::ssdp::SSDPError, ::ssdp::SSDPErrorKind);
        Hue(::philipshue::errors::HueError, ::philipshue::errors::HueErrorKind);
    }
    
    foreign_links {
        Io(::std::io::Error);
    }
    
}
