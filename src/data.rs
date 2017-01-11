// (c) 2016 Joost Yervante Damad <joost@damad.be>

/// variants of Hue structs more suitable for storage

use philipshue::hue;

/// Light in HS mode
#[derive(Debug, PartialEq)]
pub struct Hs {
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: u16,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    pub sat: u8,
}

/// Light in XY mode
#[derive(Debug, PartialEq)]
pub struct Xy {
    pub x: f32,
    pub y: f32,
}

/// Light in CT mode
#[derive(Debug, PartialEq)]
pub struct Ct {
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    pub ct: u16,
}

/// color state of the light
#[derive(Debug, PartialEq)]
pub enum ColorState {
    Hs(Hs),
    Xy(Xy),
    Ct(Ct),
}

/// state of the light
#[derive(Debug, PartialEq)]
pub struct LightState {
    /// Whether the light is on
    pub on: bool,
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    pub bri: u8,
    pub color: ColorState,
}

#[derive(Debug, PartialEq)]
/// Details about a specific light
pub struct Light {
    /// short id
    pub id: usize,
    /// The unique name given to the light
    pub name: String,
    /// The hardware model of the light
    pub modelid: String,
    /// The version of the software running on the light
    pub swversion: String,
    /// Unique ID of the device
    pub uniqueid: String,
    /// The state of the light (See `LightState` for more)
    pub state: LightState,
}

impl From<hue::LightState> for LightState {
    fn from(s: hue::LightState) -> LightState {
        let colormode = s.colormode.unwrap();
        let color = match colormode.as_str() {
            "hs" => {
                ColorState::Hs(Hs {
                    hue: s.hue.unwrap(),
                    sat: s.sat.unwrap(),
                })
            }
            "xy" => {
                ColorState::Xy(Xy {
                    x: s.xy.unwrap().0,
                    y: s.xy.unwrap().1,
                })
            }
            "ct" => ColorState::Ct(Ct { ct: s.ct.unwrap() }),
            x => panic!("unsupported colormode {}", x),
        };
        LightState {
            on: s.on,
            bri: s.bri,
            color: color,
        }
    }
}

impl Light {
    pub fn make(l: hue::Light, id: usize) -> Light {
        Light {
            id: id,
            name: l.name,
            modelid: l.modelid,
            swversion: l.swversion,
            uniqueid: l.uniqueid,
            state: l.state.into(),
        }
    }
}
