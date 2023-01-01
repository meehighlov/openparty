use std::{collections::HashMap, iter::Map};
use ureq;
use crate::telegram::models::Update;

pub struct Client {
    base_url: &'static str,
    updates_url: &'static str,
}

struct Request{
    uri: String,
    query: String,
    body: String,
}
struct Response<T>{body: T}


impl Client {

    pub fn build(token: String) -> Client {
        let mut base_url: String = "https://api.telegram.org/bot".to_string();
        base_url.push_str(&token);

        Client {
            base_url: base_url.as_str(),
            updates_url: "/getUpdates",
        }
    }

    fn make_uri<'a>(base: &'a str, tail: &'a str) -> &'a str {
        return format!("{base}{tail}").as_str();
    }

    fn add_query(url: &str, query: HashMap<String, String>) -> String {
        let mut url_with_query: String = format!("{url}");
        for (key, value) in query {
            let param: String = format!("{key}={value}");
            url_with_query.push_str(&param);
        }

        url_with_query
    }

    fn get(&self, url: &str, query: &str) {
        let response = match ureq::get(url).call() {
            
        } 
    }

    fn call(&self, method: &str, url_tail: &str, query: &str, params: HashMap<String, String>) {
        let response = match method.to_lowercase() {
            case "get" => self.get(query);
        }
        
    }

    pub fn getUpdates(&self) -> Vec<Update> {
        let updates: Vec<Update> = Vec::new();

        

        return updates;
    }
}
