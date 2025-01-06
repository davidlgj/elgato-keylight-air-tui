use isahc::{prelude::*, Request};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Light {
    pub on: i32,
    pub brightness: i32,
    pub temperature: i32,
}

impl Light {
    pub fn new(on: bool, brightness: i32, temperature: i32) -> Self {
        Light {
            on: on as i32,
            brightness,
            temperature,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lights {
    pub number_of_lights: i32,
    pub lights: Vec<Light>,
}

impl Lights {
    pub fn new(light: Light) -> Self {
        Lights {
            number_of_lights: 1,
            lights: vec![light],
        }
    }
}

pub fn fetch_light_status(ip: &str) -> Light {
    let url = format!("http://{ip}:9123/elgato/lights");
    let mut response = isahc::get(url).expect("Could not contact key light, is it on?");

    if !response.status().is_success() {
        panic!("Failed to get a successful response status! Is the key light on?");
    }
    let res: Lights = response.json().expect("Could not parse JSON");
    print!("{:?}", res);

    // There is always just one on the key light
    res.lights
        .into_iter()
        .nth(0)
        .expect("Expected to find at least one light!")
}

pub fn set_light(ip: &str, light: Light) -> Result<Light, Box<dyn std::error::Error>> {
    let url = format!("http://{ip}:9123/elgato/lights");

    let lights = Lights::new(light);
    let body = serde_json::to_string(&lights)?;

    let mut response = Request::put(url)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(1))
        .body(body)?
        .send()?;

    if !response.status().is_success() {
        panic!("Failed to get a successful response status! Is the key light on?");
    }
    let res: Lights = response.json()?;

    // There is always just one on the key light
    let light = res.lights.into_iter().nth(0).expect("Found no light");
    Ok(light)
}
