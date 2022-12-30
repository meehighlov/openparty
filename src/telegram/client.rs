use std::collections::HashMap;
use crate::telegram::models::Update;

pub struct Client {
    token: String,
    base_url: String,
    updates_url: &'static str,
}

pub struct Request {}

pub struct Response {}


impl Client {

    pub fn build(token: String, base_url: String) -> Client {
        Client {
            token,
            base_url,
            updates_url: "/getUpdates",
        }
    }

    pub fn call(url_tail: String, params: HashMap<String, String>) -> Response {

        Response {  }
    }

    pub fn getUpdates(&self) -> Vec<Update> {
        let updates = Vec::new();

        

        return updates;
    }
}
