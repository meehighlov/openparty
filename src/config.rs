use std::env;

pub struct  Config {
    pub token: String,
}

impl Config {
    pub fn build() -> Config {
        let token = env::var("OPENPARTYBOT_TOKEN").unwrap();

        Config { token }
    }
}
