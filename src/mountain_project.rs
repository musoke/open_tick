use chrono::NaiveDate;
use std::convert::TryFrom;
use url::Url;

/// A tick as recorded in an export from
/// `https://www.mountainproject.com/user/<userid>/<username>/tick-export`
#[derive(Debug)]
pub struct MountainProjectTick {
    pub date: Option<NaiveDate>,
    /// Mountain Project assigned name
    pub route: String,
    /// Mountain Project assigned grade
    pub rating: String,
    pub notes: String,
    /// URL of route on <https://www.mountainproject.com>
    pub url: Option<Url>,
    pub pitches: u8,
    pub location: String,
    pub avg_stars: f32,
    /// -1 if no rating, 1-5 otherwise
    pub your_stars: i8,
    pub style: MountainProjectStyle,
    pub lead_style: Option<MountainProjectLeadStyle>,
    /// The type of route climbed
    /// ### Examples
    /// ```txt
    /// Sport
    /// "Sport, TR"
    /// ```
    pub route_type: String,
    /// ticker's own grade, which may differ from "official" grade
    pub your_rating: String,
    /// length of route in feet
    pub length: usize,
    /// unclear meaning, u16 might suffice
    pub rating_code: u32,
}

/// Styles of ascent allowed by Mountain Project
#[non_exhaustive]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
/// .unwrap();
/// let mp_id = MountainProjectRouteId::try_from(url).unwrap();
/// assert_eq!(mp_id.0, 12321)
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
                // "v" is another possibly valid value, but then don't know if it's a route or area
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
    use std::convert::TryFrom;

    #[test]
    fn good_mp_url() {
        let id = 123456;
        let url = Url::parse(&format!(
            "https://www.mountainproject.com/route/{id}/route-name"
        ))
        .ok()
        .unwrap();
        let mp_id = MountainProjectRouteId::try_from(url);

        assert_eq!(mp_id, Ok(MountainProjectRouteId(id)))
    }

    #[test]
    fn bad_mp_route_url() {
        let id = 123456;
        let url = Url::parse(&format!(
            "https://www.mountainproject.com/area/{id}/area-name"
        ))
        .ok()
        .unwrap();
        let mp_id = MountainProjectRouteId::try_from(url);

        assert!(mp_id.is_err())
    }

    #[test]
    fn a_tick() {
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

        println!("{t:?}")
    }
}
