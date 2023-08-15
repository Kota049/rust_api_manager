#[cfg(test)]
pub mod tests;

use super::super::constants::HTTP_METHODS;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq,Serialize, Deserialize)]
pub struct Api {
    url: String,
    method: String,
    parameter: Vec<String>,
}

impl Api {
    pub fn new(url: String, method: String, parameter: Vec<String>) -> Result<Api, String> {
        let method  = method.to_uppercase();
        validate_url(&url)?;
        validate_method(&method)?;
        Ok(Api {
            url,
            method,
            parameter,
        })
    }

}

fn validate_method(method: &str)->Result<(),String> {
    if !HTTP_METHODS.iter().any(|e| e == &method) {
        return Err(String::from("Invalid Method(method should be http method[GET,POST,PUT,DELETE])"));
    }
    Ok(())
}
fn validate_url(url: &str) -> Result<(), String> {
    if !url.starts_with('/') {
        return Err(String::from("Invalid URL(url should be start with /)"));
    }
    Ok(())
}