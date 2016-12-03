// (c) 2016 Joost Yervante Damad <joost@damad.be>

/// variants of Hue structs more suitable for storage

/// Light in HS mode
#[derive(Debug, Deserialize, Serialize)]
pub struct Hs {
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    bri: u8,
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    hue: u16,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    sat: u8,
}

/// Light in XY mode
#[derive(Debug, Deserialize, Serialize)]
pub struct Xy {
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    bri: u8,
    x:f32,
    y:f32,
}

/// Light in CT mode
#[derive(Debug, Deserialize, Serialize)]
pub struct Ct {
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    bri: u8,
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    ct: u16,
}

/// State of the light
#[derive(Debug, Deserialize, Serialize)]
pub enum LightState {
    Hs(Hs),
    Xy(Xy),
    Ct(Ct),
}

#[derive(Debug, Deserialize, Serialize)]
/// Details about a specific light
pub struct Light {
    /// The unique name given to the light
    pub name: String,
    /// The hardware model of the light
    pub modelid: String,
    /// The version of the software running on the light
    pub swversion: String,
    /// Unique ID of the device
    pub uniqueid: String,
    /// The state of the light (See `LightState` for more)
    pub state: LightState
}
