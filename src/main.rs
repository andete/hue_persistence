// (c) 2016 Joost Yervante Damad <joost@damad.be>

/// Hue persistence for Philips Hue Lights

extern crate philipshue;
extern crate ssdp;
extern crate syslog;
#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate env_logger;

use std::env;
use std::collections::HashMap;
use philipshue::bridge::{Bridge, discover_upnp};
use philipshue::hue::LightCommand;

use syslog::Facility;

use error::Error;

use clap::{Arg, App};

#[derive(Debug)]
struct State {
    lights: HashMap<String, data::Light>,
    reachable: HashMap<String, bool>,
}

impl Default for State {
    fn default() -> State {
        State {
            lights: HashMap::new(),
            reachable: HashMap::new(),
        }
    }
}

fn get_bridge(username: &str) -> Result<Bridge, Error> {
    let mut ips = discover_upnp()?;
    let ip = match ips.pop() {
        Some(ip) => ip,
        None => return Err("no IP found for bridge".into()),
    };
    Ok(Bridge::new(ip, username))
}

fn is_newly_reachable(state: &mut State, id: &str, reachable: bool, name: &str) -> bool {
    let newly = match state.reachable.get(id) {
        None => {
            info!("new light: {}", id);
            false // we don't know about this light yet...
        }
        Some(&was_reachable) => {
            if was_reachable && !reachable {
                info!("light went off: {}", name);
            }
            if !was_reachable && reachable {
                info!("light now on: {}", name);
            }
            !was_reachable && reachable
        }
    };
    state.reachable.insert(id.into(), reachable);
    newly
}

fn set_light(bridge: &Bridge, state: &State, id: &str) -> Result<(), Error> {
    match state.lights.get(id) {
        None => {
            warn!("error: can't set light");
            Ok(())
        }
        Some(light) => {
            let cmd = LightCommand::default();
            let cmd = cmd.on().with_bri(light.state.bri);
            let cmd = match light.state.color {
                data::ColorState::Hs(ref hs) => cmd.with_hue(hs.hue).with_sat(hs.sat),
                data::ColorState::Xy(ref xy) => cmd.with_xy((xy.x, xy.y)),
                data::ColorState::Ct(ref ct) => cmd.with_ct(ct.ct).with_sat(254),
            };
            let resps = bridge.set_light_state(light.id, &cmd)?;
            for resp in resps {
                info!("Response: {:?}", resp);
            }
            Ok(())
        }
    }
}

fn handle_lights(state: &mut State, bridge: &Bridge) -> Result<(), Error> {
    let lights = bridge.get_all_lights()?;
    for (id, light) in lights {
        let reachable = light.state.reachable;
        // println!("{} {}", id, reachable);
        let light = data::Light::make(light, id);
        if is_newly_reachable(state, &light.uniqueid, reachable, &light.name) {
            set_light(bridge, state, &light.uniqueid)?;
        } else {
            // set or update state
            match state.lights.get(&light.uniqueid) {
                Some(old_light) => {
                    if *old_light != light {
                        info!("Updating light {}", light.name)
                    }
                },
                None => (),
            }
            state.lights.insert(light.uniqueid.clone(), light);
        }
    }
    Ok(())
}

fn main() {
    let matches = App::new("Hue Persistence")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("Persistence for Philips Hue Lights")
        .arg(Arg::with_name("syslog")
             .short("s")
             .long("syslog")
             .help("enables syslog logging"))
        .arg(Arg::with_name("username")
             .help("hue username")
             .required(true)
             .index(1))
        .get_matches();
    
    if matches.is_present("syslog") {
        let syslog = syslog::unix(Facility::LOG_USER).unwrap();
        log::set_logger(|max_level| {
            max_level.set(log::LogLevelFilter::Info);
            syslog}).unwrap();
    } else {
        env::set_var("RUST_LOG","debug");
        env_logger::init().unwrap(); 
    }

    let username = matches.value_of("username").unwrap();

    let mut state = State::default();

    loop {
        let bridge = match get_bridge(&username) {
            Ok(bridge) => bridge,
            Err(e) => {
                error!("Error finding bridge: {:?}", e);
                // try to find bridge again in 60 sec
                std::thread::sleep(std::time::Duration::new(60, 0));
                continue;
            }
        };
        loop {
            match handle_lights(&mut state, &bridge) {
                Ok(_) => (),
                Err(e) => {
                    error!("error handling lights: {:?}", e);
                    // try to find bridge again in 60 sec
                    std::thread::sleep(std::time::Duration::new(60, 0));
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::new(10, 0));
        }
    }
}

mod error;
mod data;
