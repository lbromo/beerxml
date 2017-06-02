// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::result;
use std::str::FromStr;

use error::*;

/// a yeast
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Yeast {
    /// name of the hop
    #[serde(skip)]
    pub name: String,
    /// version of the yeast format (normally 1)
    pub version: i64,
    /// yeast type
    #[serde(rename="type")]
    pub type_: YeastType,
    /// yeast form
    pub form: YeastForm,
    /// amount (liter or kg)
    pub amount: f64,
    /// if amount is in kg
    pub amount_is_weight: bool,
    /// name of the producer
    pub laboratory: Option<String>,
    /// manufacturer product id
    pub product_id: Option<String>,
    /// minimum recommended temperature for fermenting this yeast strain in degrees Celsius
    pub min_temperature: Option<f64>,
    /// maximum recommended temperature for fermenting this yeast strain in Celsius
    pub max_temperature: Option<f64>,
    /// yeast flocculation
    pub flocculation: Option<YeastFlocculation>,
    /// attenuation of the yeast in percent
    pub attenuation: Option<f64>,
    /// notes
    pub notes: Option<String>,
    /// styles or types of beer this yeast strain is best suited for
    pub best_for: Option<String>,
    /// number of times this yeast has been reused as a harvested culture.  This number should be zero if this is a product directly from the manufacturer
    pub times_cultured: Option<i64>,
    /// recommended of times this yeast can be reused (recultured from a previous batch)
    pub max_reuse: Option<i64>,
    /// flag denoting that this yeast was added for a secondary (or later) fermentation as opposed to the primary fermentation.  Useful if one uses two or more yeast strains for a single brew (eg: Lambic).  Default value is false
    pub add_to_secondary: bool,
    /// amount
    pub display_amount: Option<String>,
    /// recommended minimum temperature
    pub display_min_temp: Option<String>,
    /// recommended maximum temperature
    pub display_max_temp: Option<String>,
    /// inventory
    pub inventory: Option<String>,
    /// date the culture was made
    pub culture_date: Option<String>,
}

/// the type of a yeast
#[derive(Serialize, Deserialize, Debug)]
pub enum YeastType {
    /// ale (top-fermenting) yeast
    Ale,
    /// lager (bottom-fermenting) yeast
    Lager,
    /// wheat yeast
    Wheat,
    /// wine yeast
    Wine,
    /// champage yeast
    Champagne,
}

impl Default for YeastType {
    fn default() -> YeastType {
        YeastType::Ale
    }
}

impl FromStr for YeastType {
    type Err = Error;
    fn from_str(name: &str) -> Result<YeastType> {
        match name {
            "Ale" => Ok(YeastType::Ale),
            "Lager" => Ok(YeastType::Lager),
            "Wheat" => Ok(YeastType::Wheat),
            "Wine" => Ok(YeastType::Wine),
            "Champagne" => Ok(YeastType::Champagne),
            _ => Err(ErrorKind::ParseError("YeastType".into(), name.into()).into()),
        }
    }
}

impl fmt::Display for YeastType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastType::Ale => "Ale",
            YeastType::Lager => "Lager",
            YeastType::Wheat => "Wheat",
            YeastType::Wine => "Wine",
            YeastType::Champagne => "Champagne",
        };
        write!(f, "{}", x)
    }
}

/// the form of the yeast
#[derive(Serialize, Deserialize, Debug)]
pub enum YeastForm {
    /// liquid yeast
    Liquid,
    /// dry yeast
    Dry,
    /// slated yeast
    Slate,
    /// a yeast culture
    Culture,
}

impl Default for YeastForm {
    fn default() -> YeastForm {
        YeastForm::Liquid
    }
}
impl fmt::Display for YeastForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastForm::Liquid => "Liquid",
            YeastForm::Dry => "Dry",
            YeastForm::Slate => "Slate",
            YeastForm::Culture => "Culture",
        };
        write!(f, "{}", x)
    }
}

impl FromStr for YeastForm {
    type Err = Error;
    fn from_str(name: &str) -> Result<YeastForm> {
        match name {
            "Liquid" => Ok(YeastForm::Liquid),
            "Dry" => Ok(YeastForm::Dry),
            "Slate" => Ok(YeastForm::Slate),
            "Culture" => Ok(YeastForm::Culture),
            _ => Err(ErrorKind::ParseError("YeastForm".into(), name.into()).into()),
        }
    }
}

/// flocculation of a yeast
#[derive(Serialize, Deserialize, Debug)]
pub enum YeastFlocculation {
    /// low flocculation
    Low,
    /// medium flocculation
    Medium,
    /// high flocculation
    High,
    /// very flocculation
    #[serde(rename = "Very High")]
    VeryHigh,
}

impl Default for YeastFlocculation {
    fn default() -> YeastFlocculation {
        YeastFlocculation::Low
    }
}

impl fmt::Display for YeastFlocculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastFlocculation::Low => "Low",
            YeastFlocculation::Medium => "Medium",
            YeastFlocculation::High => "High",
            YeastFlocculation::VeryHigh => "Very High",
        };
        write!(f, "{}", x)
    }
}

impl FromStr for YeastFlocculation {
    type Err = Error;
    fn from_str(name: &str) -> Result<YeastFlocculation> {
        match name {
            "Low" => Ok(YeastFlocculation::Low),
            "Medium" => Ok(YeastFlocculation::Medium),
            "High" => Ok(YeastFlocculation::High),
            "Very High" => Ok(YeastFlocculation::VeryHigh),
            _ => Err(ErrorKind::ParseError("YeastFlocculation".into(), name.into()).into()),
        }
    }
}