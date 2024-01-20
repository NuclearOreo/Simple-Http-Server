use anyhow::{Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default, Debug)]
pub enum Method {
    #[default]
    GET,
    POST,
}

impl FromStr for Method {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        let input = input.to_lowercase();
        match &input[..] {
            "get" => Ok(Method::GET),
            "post" => Ok(Method::POST),
            _ => Err(Error::msg("Failed to Parse Method")),
        }
    }
}

#[derive(Default, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn parse_request(s: String) -> Result<Request> {
    let s = s.trim_matches(char::from(0));
    let lines: Vec<&str> = s.split("\r\n").collect();
    let start_line_tokens: Vec<&str> = lines[0].split(" ").collect();

    if lines.len() == 0 || start_line_tokens.len() < 3 {
        panic!("Bad Request");
    }

    let (method, path, version) = (
        Method::from_str(start_line_tokens[0])?,
        start_line_tokens[1],
        start_line_tokens[2],
    );

    let mut headers = HashMap::new();
    let mut body = String::new();
    for i in 1..lines.len() {
        let tokens: Vec<&str> = lines[i].split(": ").collect();
        if tokens.len() >= 2 {
            headers.insert(tokens[0].to_string(), tokens[1].to_string());
        }
        if tokens.len() == 1 {
            body += tokens[0];
        }
    }

    Ok(Request {
        method,
        path: path.to_string(),
        version: version.to_string(),
        headers,
        body,
    })
}
