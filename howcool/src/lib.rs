//! A simple program to check how cool
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

static URL: &str = "https://data.sensor.community/airrohr/v1/sensor/";
/// A Vec of Datapoint structs
pub type Station = Vec<Datapoint>;

/// A struct to hold received data-points
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datapoint {
    pub sensordatavalues: Vec<Sensordatavalue>,
    pub timestamp: String,
}
/// A struct to hold returned values for sensor
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensordatavalue {
    #[serde(rename = "value_type")]
    pub value_type: String,
    pub value: Value,
}

/// Holds parsed command line argument values
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t = 59497)]
    pub station: u32,

}

impl Args {
    /// method to print constructed url
    pub fn print(&self) {
        println!("{}", build_url(&self));
    }
}
/// private function that does url construct work
fn build_url(args: &Args) -> String {
        let mut full_url: String = URL.to_string();
        full_url.push_str(&args.station.to_string());
        full_url.push('/');
        full_url
    }
/// public function that gets response
pub fn response(args: &Args) -> Station {
    let request_url = build_url(args);
    let station: Station = reqwest::blocking::get(request_url).unwrap().json().unwrap();
    station
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn url_verify() {
        let args = Args {station: 11111};
        let built_url = build_url(&args);
        let expected_url = String::from("https://data.sensor.community/airrohr/v1/sensor/11111/");
        assert_eq!(built_url, expected_url);
    }
}
