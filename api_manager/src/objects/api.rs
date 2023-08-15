#[cfg(test)]
pub mod tests;

#[derive(Debug, PartialEq)]
pub struct Api {
    url: String,
    method: String,
    parameter: Vec<String>,
}

impl Api {
    pub fn new(url: String, method: String, parameter: Vec<String>) -> Result<Api, String> {
        validate_url(&url)?;
        Ok(Api {
            url,
            method,
            parameter,
        })
    }
}

fn validate_url(url: &str) -> Result<(), String> {
    if !url.starts_with('/') {
        return Err(String::from("Invalid URL(url should be start with /)"));
    }
    Ok(())
}