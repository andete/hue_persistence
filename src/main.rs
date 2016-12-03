// (c) 2016 Joost Yervante Damad <joost@damad.be>

extern crate philips_hue_client;
use std::env;
use philips_hue_client::bridge;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let mut ips = bridge::discover_upnp().unwrap();
    let ip = ips.pop().unwrap();
    let bridge = bridge::Bridge::new(ip, &*args[1]);

    match bridge.get_all_lights() {
        Ok(lights) => {
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
        }
        Err(err) => panic!(err),
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
