use chrono::NaiveDate;
use std::convert::TryFrom;
use url::Url;

/// A tick as recorded in an export from
/// `https://www.mountainproject.com/user/<userid>/<username>/tick-export`
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MountainProjectTick {
    #[serde(rename = "Date")]
    pub date: Option<NaiveDate>,

    /// Mountain Project assigned name
    #[serde(rename = "Route")]
    pub route: String,

    /// Mountain Project assigned grade
    #[serde(rename = "Rating")]
    pub rating: String,

    #[serde(rename = "Notes")]
    pub notes: String,

    /// URL of route on <https://www.mountainproject.com>
    #[serde(rename = "URL")]
    pub url: Option<Url>,

    #[serde(rename = "Pitches")]
    pub pitches: u8,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "Avg Stars")]
    pub avg_stars: f32,

    /// -1 if no rating, 1-5 otherwise
    #[serde(rename = "Your Stars")]
    pub your_stars: i8,

    #[serde(rename = "Style")]
    pub style: MountainProjectStyle,

    #[serde(rename = "Lead Style")]
    pub lead_style: Option<MountainProjectLeadStyle>,

    /// The type of route climbed
    ///
    /// ### Examples
    /// ```txt
    /// Sport
    /// "Sport, TR"
    /// ```
    #[serde(rename = "Route Type")]
    pub route_type: String,

    /// ticker's own grade, which may differ from "official" grade
    #[serde(rename = "Your Rating")]
    pub your_rating: String,

    /// length of route in feet
    #[serde(rename = "Length")]
    pub length: usize,

    /// unclear meaning, u16 might suffice
    #[serde(rename = "Rating Code")]
    pub rating_code: u32,
}

/// Styles of ascent allowed by Mountain Project
#[non_exhaustive]
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MountainProjectStyle {
    /// only for boulders
    Attempt,
    /// only for boulders
    Flash,
    /// not for boulders
    Follow,
    /// not for boulders
    Lead,
    /// only for boulders
    Send,
    /// not for boulders
    Solo,
    /// not for boulders
    TR,
}

/// Sub-styles for lead ascents
#[non_exhaustive]
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum MountainProjectLeadStyle {
    FellHung,
    Flash,
    Onsight,
    Pinkpoint,
    Redpoint,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum MountainProjectRouteType {
    Sport,
    TR,
    Trad,
    Unknown,
}

impl From<String> for MountainProjectRouteType {
    fn from(value: String) -> Self {
        match value {
            _ => MountainProjectRouteType::Unknown,
        }
    }
}

/// ID of a route in Mountain Project's database
///
/// # Examples
///
/// ```
/// use url::Url;
/// use open_tick::mountain_project::MountainProjectRouteId;
/// let url = Url::parse("https://www.mountainproject.com/route/12321/route-name")
/// .ok()
/// .expect("valid URL");
///
/// let mp_id = MountainProjectRouteId::try_from(url).expect("valid route url");
/// assert_eq!(mp_id, MountainProjectRouteId(12321))
/// ```
#[derive(Debug, PartialEq)]
pub struct MountainProjectRouteId(pub usize);

impl TryFrom<Url> for MountainProjectRouteId {
    type Error = MountainProjectIdConversionError;

    fn try_from(value: Url) -> Result<Self, Self::Error> {
        if value.domain() != Some("www.mountainproject.com") {
            Err(MountainProjectIdConversionError::WrongDomain)
        } else {
            let mut path = value
                .path_segments()
                .ok_or(MountainProjectIdConversionError::WrongDomain)?;

            match path.next() {
                Some("route") => {}
                // "v" is another possibly valid value, but then one can't know if the url is for a
                // route or an area.  MP doesn't seem to use these urls in CSV logbooks.
                Some("v") => Err(MountainProjectIdConversionError::BadPath)?,
                Some(_) => Err(MountainProjectIdConversionError::BadPath)?,
                None => Err(MountainProjectIdConversionError::BadPath)?,
            }

            let id = path
                .next()
                .ok_or(MountainProjectIdConversionError::BadPath)?
                .parse::<usize>()
                .map_err(|_| -> MountainProjectIdConversionError {
                    MountainProjectIdConversionError::BadPath
                })?;

            Ok(MountainProjectRouteId(id))
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum MountainProjectIdConversionError {
    WrongDomain,
    BadPath,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::convert::TryFrom;

    #[test]
    fn mp_route_url_good() -> Result<(), MountainProjectIdConversionError> {
        let id = 123456;
        let url = Url::parse(&format!(
            "https://www.mountainproject.com/route/{id}/route-name"
        ))
        .ok()
        .expect("valid url");
        let mp_id = MountainProjectRouteId::try_from(url)?;

        assert_eq!(mp_id, MountainProjectRouteId(id));
        Ok(())
    }

    #[test]
    fn mp_route_url_is_area() {
        let id = 123456;
        let url = Url::parse(&format!(
            "https://www.mountainproject.com/area/{id}/area-name"
        ))
        .ok()
        .expect("valid url");

        let mp_id = MountainProjectRouteId::try_from(url);

        assert_eq!(mp_id, Err(MountainProjectIdConversionError::BadPath))
    }

    #[test]
    fn mp_route_url_wrong_domain() {
        let id = 123456;
        let url = Url::parse(&format!(
            "https://www.projectmountain.com/route/{id}/route-name"
        ))
        .ok()
        .expect("valid url");

        let mp_id = MountainProjectRouteId::try_from(url);

        assert_eq!(mp_id, Err(MountainProjectIdConversionError::WrongDomain))
    }

    #[test]
    fn init_tick() {
        let t: MountainProjectTick = MountainProjectTick {
            date: NaiveDate::from_ymd_opt(2020, 1, 1),
            route: "a route name".to_string(),
            rating: "V2".to_string(),
            notes: "fund route".to_string(),
            url: Url::parse("https://www.mountainproject.com/route/123456/route-name").ok(),
            pitches: 1,
            location: "A place > the crag".to_string(),
            avg_stars: 3.2,
            your_stars: 3,
            style: MountainProjectStyle::TR,
            lead_style: Some(MountainProjectLeadStyle::FellHung),
            route_type: "\"Trad, TR\"".to_string(),
            your_rating: "5.10".to_string(),
            length: 10,
            rating_code: 20008,
        };

        println!("{t:?}");
    }

    #[test]
    fn from_csv() -> Result<(), Box<dyn std::error::Error>> {
        let mp_csv = r#"Date,Route,Rating,Notes,URL,Pitches,Location,"Avg Stars","Your Stars",Style,"Lead Style","Route Type","Your Rating",Length,"Rating Code"
2023-06-01,"Route Name",V1,,https://www.mountainproject.com/route/271828/route-name,1,"Area > Crag",2.5,-1,Send,,Boulder,,10,20300
"#;

        let mut ticks = Vec::new();
        let mut reader = csv::Reader::from_reader(mp_csv.as_bytes());

        for record in reader.deserialize() {
            let record: MountainProjectTick = record?;

            assert_eq!(record.date, NaiveDate::from_ymd_opt(2023, 06, 01));
            assert_eq!(record.route, "Route Name");
            assert_eq!(record.length, 10);

            ticks.push(record);
        }

        let mut writer = csv::Writer::from_writer(vec![]);
        for record in ticks.iter() {
            writer.serialize(record)?;
        }

        // Check that record serializes to original, modulo quoting strings with spaces
        assert_eq!(
            mp_csv.replace("\"", ""),
            String::from_utf8(writer.into_inner()?).unwrap()
        );

        Ok(())
    }
}
