// Contains various functions used through heycli
use std::io::{stdin, stdout, Write};
use reqwest::blocking;
use crate::API_URL;
use serde_json::Value;

// Grab user input
pub fn get_input() -> String {
    let mut input = String::new();
            
    let _ = stdout()
        .flush();
    
    stdin()
        .read_line(&mut input)
        .expect("Unable to read user input");
        
    input.trim().to_string()
}

// Validate API Key
pub fn is_key_valid(key: &str) -> bool {
    let url = format!("{API_URL}api/account_loginkey?query={key}&error_boolean=true");
    let api_data = get_hey_cafe_api(url);
    let error = api_data["system_api_error"].as_bool().unwrap();
    
    if error {
        false
    } else {
        true
    }
}

// Check if the key is set in general
pub fn is_key_set() -> bool {
    match std::env::var("HEYCAFE_KEY") {
        Ok(_) => true,
        Err(_) => false
    }
}

// Grab API data as a Value from Hey.Cafe
pub fn get_hey_cafe_api(mut url: String) -> Value {
    if let Ok(key) = std::env::var("HEYCAFE_KEY") {
        url = format!("{url}&auth={key}");
    }
    
    let raw_api_data = match blocking::get(url) {
        Ok(data) => data.text().unwrap_or(String::new()),
        Err(e) => panic!("ERROR: Unable to fetch API information! {e}")  
    };
    
    let api_data: Value = match serde_json::from_str(&raw_api_data) {
        Ok(data) => data,
        Err(e) => panic!("ERROR: Unable to process API data! {e}")    
    };
    
    api_data
}

// Send a post request to Hey.Cafe
pub fn post_hey_cafe_api(mut url: String) {
    if let Ok(key) = std::env::var("HEYCAFE_KEY") {
        url = format!("{url}&auth={key}");
    }
    
    let client = blocking::Client::new();
    if let Err(e) = client.post(url).send() {
        panic!("ERROR: Unable to send POST request to API! {e}");
    }
}