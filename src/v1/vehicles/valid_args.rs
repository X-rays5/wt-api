pub use std::collections::HashSet;
pub use std::str::FromStr;
use lazy_static::lazy_static;

pub fn string_name_format(s: &str) -> String {
    let mut str_copy = s.as_bytes().to_vec();
    str_copy = str_copy.to_ascii_lowercase();
    str_copy[0] = str_copy[0].to_ascii_uppercase();
    let str_copy = String::from_utf8(str_copy).unwrap();

    str_copy
}

#[derive(Eq, Hash, PartialEq)]
pub enum ECountries {
    Usa,
    Germany,
    Ussr,
    Britain,
    Japan,
    China,
    Italy,
    France,
    Sweden,
    Israel
}

impl FromStr for ECountries {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = string_name_format(s);
        let s = s.as_str();

        match s {
            "Usa" => Ok(ECountries::Usa),
            "Germany" => Ok(ECountries::Germany),
            "Ussr" => Ok(ECountries::Ussr),
            "Britain" => Ok(ECountries::Britain),
            "Japan" => Ok(ECountries::Japan),
            "China" => Ok(ECountries::China),
            "Italy" => Ok(ECountries::Italy),
            "France" => Ok(ECountries::France),
            "Sweden" => Ok(ECountries::Sweden),
            "Israel" => Ok(ECountries::Israel),
            _ => {Err(())}
        }
    }
}

pub fn is_country(country: &str) -> bool {
    let country_tmp = string_name_format(country);
    match ECountries::from_str(&*country_tmp) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum EVehiclesCategories {
    Ground,
    Planes,
    Helicopters,
    Naval,
}

impl FromStr for EVehiclesCategories {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = string_name_format(s);
        let s= s.as_str();

        match s {
            "Ground" => Ok(EVehiclesCategories::Ground),
            "Planes" => Ok(EVehiclesCategories::Planes),
            "Helicopters" => Ok(EVehiclesCategories::Helicopters),
            "Naval" => Ok(EVehiclesCategories::Naval),
            _ => {Err(())}
        }
    }
}

pub fn is_category(category: &str) -> bool {
    let category_tmp = string_name_format(category);
    match EVehiclesCategories::from_str(&*category_tmp) {
        Ok(_) => true,
        Err(_) => false
    }
}

// valid categories per country
lazy_static! {
    pub static ref USA_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref GERMANY_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref USSR_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref BRITAIN_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref JAPAN_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref CHINA_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);

        set
    };
    pub static ref ITALY_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);
        set.insert(EVehiclesCategories::Naval);

        set
    };
    pub static ref FRANCE_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);

        set
    };
    pub static ref SWEDEN_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);

        set
    };
    pub static ref ISREAL_VEHICLES: HashSet<EVehiclesCategories> = {
        let mut set = HashSet::new();
        set.insert(EVehiclesCategories::Ground);
        set.insert(EVehiclesCategories::Planes);
        set.insert(EVehiclesCategories::Helicopters);

        set
    };
}

pub fn country_has_category(country: ECountries, category: EVehiclesCategories) -> bool {
    match country {
        ECountries::Usa => {USA_VEHICLES.contains(&category)}
        ECountries::Germany => {GERMANY_VEHICLES.contains(&category)}
        ECountries::Ussr => {USSR_VEHICLES.contains(&category)}
        ECountries::Britain => {BRITAIN_VEHICLES.contains(&category)}
        ECountries::Japan => {JAPAN_VEHICLES.contains(&category)}
        ECountries::China => {CHINA_VEHICLES.contains(&category)}
        ECountries::Italy => {ITALY_VEHICLES.contains(&category)}
        ECountries::France => {FRANCE_VEHICLES.contains(&category)}
        ECountries::Sweden => {SWEDEN_VEHICLES.contains(&category)}
        ECountries::Israel => {ISREAL_VEHICLES.contains(&category)}
    }
}