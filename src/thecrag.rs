use chrono::NaiveDateTime;
// use std::convert::TryFrom;
use url::Url;

/// A tick as recorded in an export from
/// `https://www.thecrag.com/climber/<username>/logbook-csv`
#[derive(Debug)]
pub struct TheCragTick<'a> {
    ///
    pub route_name: &'a str,
    ///
    pub ascent_label: &'a str,
    ///
    pub ascent_id: TheCragAscentId,
    ///
    pub ascent_link: Url,
    ///
    pub ascent_type: TheCragAscentType,
    /// the route grade as recorded in theCrag
    pub route_grade: &'a str,
    /// the route grade as recorded by ticker
    pub ascent_grade: &'a str,
    /// the gear style as recorded in theCrag
    pub route_gear_style: TheCragGearStyle,
    /// the gear style as climbed
    pub ascent_gear_style: TheCragGearStyle,
    /// height in meters
    pub route_height: f32,
    /// height in meters
    ///
    /// May differ from route height
    pub ascent_height: f32,
    ///
    pub number_ascents: usize,
    ///
    pub route_stars: &'a str,
    ///
    pub route_id: TheCragRouteId,
    ///
    pub route_link: Url,
    ///
    pub country: &'a str,
    /// url of parent country in theCrag
    pub country_link: Url,
    ///
    pub crag_name: &'a str,
    /// url of crag in theCrag
    pub crag_link: Url,
    /// hierarchy of areas above route
    pub crag_path: &'a str,
    /// people climbed with
    pub with: &'a str,
    ///
    pub comment: &'a str,
    ///
    pub quality: &'a str,
    ///
    pub ascent_date: NaiveDateTime,
    ///
    pub log_date: NaiveDateTime,
    ///
    pub shot: u16,
}

/// Gear styles allowed by theCrag
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum TheCragGearStyle {
    Aid,
    Boulder,
    Sport,
    Trad,
}

/// Ascent types allowed by theCrag
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum TheCragAscentType {
    Flash,
    Hangdog,
    Onsight,
    Pinkpoint,
    Send,
    Redpoint,
    Repeat,
}

/// ID of a route in theCrag's database
///
#[derive(Debug, PartialEq)]
pub struct TheCragRouteId(pub usize);

/// ID of an ascent in theCrag's database
///
#[derive(Debug, PartialEq)]
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
