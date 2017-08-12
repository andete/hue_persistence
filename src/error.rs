// (c) 2016 Joost Yervante Damad <joost@damad.be>

error_chain! {

    links {
        Hue(::philipshue::errors::HueError, ::philipshue::errors::HueErrorKind);
        SSDP(::ssdp::SSDPError, ::ssdp::SSDPErrorKind);
    }
    
    foreign_links {
        Io(::std::io::Error);
        Timer(::tokio_timer::TimerError);
    }
    
}
