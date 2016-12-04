// (c) 2016 Joost Yervante Damad <joost@damad.be>
#![feature(proc_macro)]

extern crate philipshue;
extern crate ssdp;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::collections::HashMap;
use philipshue::bridge::{Bridge, discover_upnp};
use philipshue::hue::LightCommand;

use error::Error;

#[derive(Debug)]
struct State {
    lights: HashMap<String, data::Light>,
    reachable: HashMap<String, bool>,
    last_stored: Option<String>,
}

impl Default for State {
    fn default() -> State {
        State {
            lights:HashMap::new(),
            reachable:HashMap::new(),
            last_stored:None,
        }
    }
}

fn get_bridge(username:&str) -> Result<Bridge,Error> {
    let mut ips = discover_upnp()?;
    let ip = ips.pop().unwrap();
    Ok(Bridge::new(ip, username))
}

fn is_newly_reachable(state:&mut State, id:&String, reachable:bool, name:&String) -> bool {
    let newly = match state.reachable.get(id) {
        None => {
            println!("new light: {}", id);
            false // we don't know about this light yet...
        },
        Some(&was_reachable) => {
            if was_reachable && !reachable {
                println!("light went off: {}", name);
            }
            if !was_reachable && reachable {
                println!("light now on: {}", name);
            }
            !was_reachable && reachable
        },
    };
    state.reachable.insert(id.clone(), reachable);
    newly
}

fn set_light(bridge:&Bridge, state:&State, id:&String) -> Result<(),Error> {
    match state.lights.get(id) {
        None => {
            println!("error: can't set light");
            Ok(())
        },
        Some(light) => {
            let cmd = LightCommand::default();
            let cmd = cmd.on().with_bri(light.state.bri);
            let cmd = match light.state.color {
                data::ColorState::Hs(ref hs) => {
                    cmd.with_hue(hs.hue).with_sat(hs.sat)
                },
                data::ColorState::Xy(ref xy) => {
                    cmd.with_xy((xy.x, xy.y))
                },
                data::ColorState::Ct(ref ct) => {
                    cmd.with_ct(ct.ct).with_sat(254)
                },
            };
            let resps = bridge.set_light_state(light.id, &cmd)?;
            for resp in resps.into_iter() {
                if let Some(success) = resp.success{
                    println!("Success: {:?}", success)
                }else if let Some(err) = resp.error{
                    println!("Error: {:?}", err);
                }
            }
            Ok(())
        }
    }
}

fn store_lights(state:&mut State) -> Result<(), Error> {
    let stored = serde_json::to_string(&state.lights)?;
    let store = match state.last_stored {
        None => {
            println!("new to store: {}", stored);
            true
        },
        Some(ref old) => {
            if old.as_str() == stored.as_str() {
                //println!("no new state");
                false
            } else {
                println!("store update: {}", stored);
                true
            }
        }
    };
    if store {
        state.last_stored = Some(stored);
    }
    Ok(())
}

fn handle_lights(state:&mut State, bridge:&Bridge) -> Result<(),Error> {
    let lights = bridge.get_all_lights()?;
    let mut store = true;
    for (id,light) in lights.into_iter() {
        let reachable = light.state.reachable;
        //println!("{} {}", id, reachable);
        let light = data::Light::make(light, id);
        if is_newly_reachable(state, &light.uniqueid, reachable, &light.name) {
            set_light(bridge, state, &light.uniqueid)?;
            store = false;
        } else {
            state.lights.insert(light.uniqueid.clone(), light);
        }
    }
    if store { // only store when we didn't change a light!
        store_lights(state)?
    }
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

    let mut state = State::default();
    
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
            match handle_lights(&mut state, &bridge) {
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

mod error;
mod data;
