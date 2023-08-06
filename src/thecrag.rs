use chrono::{DateTime,Utc};
// use std::convert::TryFrom;
use url::Url;

/// A tick as recorded in an export from
/// `https://www.thecrag.com/climber/<username>/logbook-csv`
#[derive(Debug, serde::Deserialize)]
pub struct TheCragTick {
    #[serde(rename = "Route Name")]
    pub route_name: String,

    #[serde(rename = "Ascent Label")]
    pub ascent_label: String,

    #[serde(rename = "Ascent ID")]
    pub ascent_id: TheCragAscentId,

    #[serde(rename = "Ascent Link")]
    pub ascent_link: Url,

    #[serde(rename = "Ascent Type")]
    pub ascent_type: TheCragAscentType,

    /// the route grade as recorded in theCrag
    #[serde(rename = "Route Grade")]
    pub route_grade: String,

    /// the route grade as recorded by ticker
    #[serde(rename = "Ascent Grade")]
    pub ascent_grade: String,

    /// the gear style as recorded in theCrag
    #[serde(rename = "Route Gear Style")]
    pub route_gear_style: TheCragGearStyle,

    /// the gear style as climbed
    #[serde(rename = "Ascent Gear Style")]
    pub ascent_gear_style: TheCragGearStyle,

    /// height in meters
    #[serde(rename = "Route Height")]
    pub route_height: String,

    /// height in meters
    ///
    /// May differ from route height
    #[serde(rename = "Ascent Height")]
    pub ascent_height: String,

    #[serde(rename = "# Ascents")]
    pub number_ascents: usize,

    #[serde(rename = "Route Stars")]
    pub route_stars: String,

    #[serde(rename = "Route ID")]
    pub route_id: TheCragRouteId,

    #[serde(rename = "Route Link")]
    pub route_link: Url,

    #[serde(rename = "Country")]
    pub country: String,

    /// url of parent country in theCrag
    #[serde(rename = "Country Link")]
    pub country_link: Url,

    #[serde(rename = "Crag Name")]
    pub crag_name: String,

    /// url of crag in theCrag
    #[serde(rename = "Crag Link")]
    pub crag_link: Url,

    /// hierarchy of areas above route
    #[serde(rename = "Crag Path")]
    pub crag_path: String,

    /// people climbed with
    #[serde(rename = "With")]
    pub with: String,

    #[serde(rename = "Comment")]
    pub comment: String,

    #[serde(rename = "Quality")]
    pub quality: String,

    #[serde(rename = "Ascent Date")]
    pub ascent_date: DateTime<Utc>,

    #[serde(rename = "Log Date")]
    pub log_date: DateTime<Utc>,

    #[serde(rename = "Shot")]
    pub shot: Option<u16>,
}

/// Gear styles allowed by theCrag
#[non_exhaustive]
#[derive(Debug, PartialEq, serde::Deserialize)]
pub enum TheCragGearStyle {
    Aid,
    Boulder,
    Sport,
    Trad,
}

/// Ascent types allowed by theCrag
#[non_exhaustive]
#[derive(Debug, PartialEq, serde::Deserialize)]
pub enum TheCragAscentType {
    Flash,
    Hangdog,
    Onsight,

    #[serde(rename = "Pink point")]
    Pinkpoint,

    Send,

    #[serde(rename = "Red point")]
    Redpoint,
    Repeat,
}

/// ID of a route in theCrag's database
///
#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct TheCragRouteId(pub usize);

/// ID of an ascent in theCrag's database
///
#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct TheCragAscentId(pub usize);

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::convert::TryFrom;

//     #[test]
//     fn a_tick() {
//         let t = TheCragTick {
//         };

//         println!("{t:?}")
//     }
// }
