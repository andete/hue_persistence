// (c) 2016-2017 Joost Yervante Damad <joost@damad.be>

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
extern crate futures;
extern crate tokio_timer;

use std::env;
use std::time::Duration;
use std::collections::HashMap;
use philipshue::bridge::{Bridge, discover_upnp};
use philipshue::hue::LightCommand;
use philipshue::network::Core;
use futures::{Future, Stream};
use futures::future::Either;

use syslog::Facility;

use error::{Error, Result};

use clap::{Arg, App};

pub type MyFuture<'a, T> = Box<Future<Item = T, Error = Error> + 'a>;

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

fn get_bridge(core:&Core, username: &str) -> Result<Bridge> {
    let mut ips = discover_upnp()?;
    let ip = match ips.pop() {
        Some(ip) => ip,
        None => return Err("no IP found for bridge".into()),
    };
    let bridge = Bridge::new(core, ip, username);
    Ok(bridge)
}

fn is_newly_reachable(state: &mut State, id: &str, reachable: bool, name: &str) -> bool {
    let newly = match state.reachable.get(id) {
        None => {
            warn!("new light: {}", id);
            false // we don't know about this light yet...
        }
        Some(&was_reachable) => {
            if was_reachable && !reachable {
                warn!("light went off: {}", name);
            }
            if !was_reachable && reachable {
                warn!("light now on: {}", name);
            }
            !was_reachable && reachable
        }
    };
    state.reachable.insert(id.into(), reachable);
    newly
}

fn set_light<'a>(bridge: &'a Bridge, state: &State, id: &str) -> MyFuture<'a, ()> {
    match state.lights.get(id) {
        None => {
            warn!("error: can't set light");
            return Box::new(futures::future::ok(()))
        }
        Some(light) => {
            let cmd = LightCommand::default();
            let cmd = cmd.on().with_bri(light.state.bri);
            let cmd = match light.state.color {
                data::ColorState::Hs(ref hs) => cmd.with_hue(hs.hue).with_sat(hs.sat),
                data::ColorState::Xy(ref xy) => cmd.with_xy((xy.x, xy.y)),
                data::ColorState::Ct(ref ct) => cmd.with_ct(ct.ct).with_sat(254),
            };
            let f = bridge.set_light_state(light.id, &cmd)
                .from_err::<Error>()
                .and_then(|resps| {
                    for resp in resps {
                        warn!("Response: {:?}", resp);
                    }
                    futures::future::ok(())
                });
            return Box::new(f)
        }
    };
}

fn handle_lights<'a>(state: &'a mut State, bridge: &'a Bridge) -> MyFuture<'a, ()> {
    let f = bridge.get_all_lights()
        .from_err::<Error>()
        .and_then(move |lights| {
            let stream = futures::stream::iter(lights.into_iter().map(|l| Ok(l)));
        stream.for_each(move |(id, light)| {
            let reachable = light.state.reachable;
            // println!("{} {}", id, reachable);
            let light = data::Light::make(light, id);
            if is_newly_reachable(state, &light.uniqueid, reachable, &light.name) {
                set_light(bridge, state, &light.uniqueid)
            } else {
                // set or update state
                match state.lights.get(&light.uniqueid) {
                    Some(old_light) => {
                        if *old_light != light {
                            warn!("Updating light {}", light.name)
                        }
                    },
                    None => (),
                }
                state.lights.insert(light.uniqueid.clone(), light);
                Box::new(futures::future::ok(()))
            }
        })
    });
    Box::new(f)
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
            max_level.set(log::LogLevelFilter::Warn);
            syslog}).unwrap();
    } else {
        env::set_var("RUST_LOG","info, hyper=warn");
        env_logger::init().unwrap(); 
    }

    let username = matches.value_of("username").unwrap();


    let mut core = Core::new().unwrap();
    loop {
        let bridge = match get_bridge(&core, &username) {
            Ok(cb) => cb,
            Err(e) => {
                error!("Error finding bridge: {:?}", e);
                // try to find bridge again in 60 sec
                std::thread::sleep(std::time::Duration::new(60, 0));
                continue;
            }
        };
        let mut state = State::default();
        loop {
            // using select2 with a sleep
            // instead of a tokio_timer::Timer::timeout()
            // because of timeout having an unwieldable error
            // type signature
            let timeout = tokio_timer::Timer::default();
            let ts_f = timeout.sleep(Duration::new(10,0))
                .from_err::<Error>();
            let handle_f = handle_lights(&mut state, &bridge);
            let future = handle_f.select2(ts_f)
                .then(|res| {
                    match res {
                        Ok(Either::A(_)) => futures::future::ok(()).boxed(),
                        Ok(Either::B(_)) => futures::future::result(Err("timeout".into())).boxed(),
                        Err(Either::A((e, _))) => futures::future::err(e).boxed(),
                        Err(Either::B((e, _))) => futures::future::err(e).boxed(),
                    }
                })
                .and_then(|_| {
                    let timer = tokio_timer::Timer::default();
                    timer.sleep(Duration::new(10,0))
                        .from_err::<Error>()
                })
                .or_else(|e| {
                    error!("error handling lights: {:?}", e);
                    // try to find bridge again in 60 sec
                    let timer = tokio_timer::Timer::default();
                    timer.sleep(Duration::new(60,0))
                        .from_err::<Error>()
                });
            core.run(future).unwrap();
        }
    }
}

mod error;
mod data;
