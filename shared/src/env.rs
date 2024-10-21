use std::env;

use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Deveropment,
    Production,
}

pub fn which() -> Environment {
    #[cfg(debug_assertions)]
    let defaut_env = Environment::Deveropment;
    #[cfg(not(debug_assertions))]
    let defaut_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => defaut_env,
        Ok(v) => v.parse().unwrap_or(defaut_env),
    }
}
