// (c) 2016 Joost Yervante Damad <joost@damad.be>

extern crate philips_hue_client;
extern crate ssdp;
use std::env;
use philips_hue_client::bridge::{Bridge, discover_upnp};

use error::Error;

fn get_bridge(username:&str) -> Result<Bridge,Error> {
    let mut ips = discover_upnp()?;
    let ip = ips.pop().unwrap();
    Ok(Bridge::new(ip, username))
}

fn handle_lights(bridge:&Bridge) -> Result<(),Error> {
    let lights = bridge.get_all_lights()?;
    println!("id name       on  bri   hue sat  temp alert   effect    mode      reachable");
    for (id,light) in lights.iter() {
        println!("{:2} {:10} {:3} {:3} {:5} {:3} {:4}K {:7} {:9} {:9} {:8}",
                 id,
                 light.name,
                 if light.state.on { "on" } else { "off" },
                 light.state.bri,
                 Show(&light.state.hue),
                 Show(&light.state.sat),
                 Show(&light.state.ct.map(|k| 1000000u32 / (k as u32))),
                 light.state.alert,
                 Show(&light.state.effect),
                 Show(&light.state.colormode),
                 light.state.reachable,
                 //Show(&light.state.xy),
        );
    }
    println!();
    Ok(())
}

fn main() {
    // TODO syslog logging
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }

    let username = args[1].clone();

    // TODO: error handling
    loop {
        let bridge = match get_bridge(&username) {
            Ok(bridge) => bridge,
            Err(e) => {
                println!("Error finding bridge: {:?}", e);
                // try to find bridge again in 60 sec
                std::thread::sleep(std::time::Duration::new(60,0));
                continue;
            }
        };
        loop {
            match handle_lights(&bridge) {
                Ok(_) => (),
                Err(e) => {
                    println!("error handling lights: {:?}", e);
                    // try to find bridge again in 60 sec
                    std::thread::sleep(std::time::Duration::new(60,0));
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::new(10,0));
        }
    }
}

use std::fmt::{self, Display, Debug};

struct Show<'a, T: 'a>(&'a Option<T>);

impl<'a, T: 'a + Display> Display for Show<'a, T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self.0{
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f)
        }
    }
}

impl<'a, T: 'a + Debug> Debug for Show<'a, T>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self.0{
            Some(ref x) => x.fmt(f),
            _ => Display::fmt("N/A", f)
        }
    }
}

mod error;
