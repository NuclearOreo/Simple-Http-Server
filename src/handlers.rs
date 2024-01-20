use anyhow::Result;
use std::collections::HashMap;
use std::fs;

const VERSION: &str = "HTTP/1.1";
const OK: &str = "200 OK";
const CREATED: &str = "201 Created";
const NOT_FOUND: &str = "404 Not Found";
const BAD_REQUEST: &str = "400 Bad Request";
const INTERNAL_SERVER_ERROR: &str = "500 Internal Server Error";
const DELIMITER: &str = "\r\n";

pub fn ok_response() -> Vec<u8> {
    vec![VERSION, " ", OK, DELIMITER, DELIMITER]
        .join("")
        .as_bytes()
        .to_vec()
}

pub fn not_found_response() -> Vec<u8> {
    vec![VERSION, " ", NOT_FOUND, DELIMITER, DELIMITER]
        .join("")
        .as_bytes()
        .to_vec()
}

pub fn bad_request_response() -> Vec<u8> {
    vec![VERSION, " ", BAD_REQUEST, DELIMITER, DELIMITER]
        .join("")
        .as_bytes()
        .to_vec()
}

pub fn internal_server_error_response() -> Vec<u8> {
    vec![VERSION, " ", INTERNAL_SERVER_ERROR, DELIMITER, DELIMITER]
        .join("")
        .as_bytes()
        .to_vec()
}

pub fn echo(s: &Vec<&str>) -> Vec<u8> {
    let mut buffer = vec![];
    buffer.extend(VERSION.as_bytes());
    buffer.extend(b" ");
    buffer.extend(OK.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(b"Content-Type: text/plain");
    buffer.extend(DELIMITER.as_bytes());
    let content = &s[2..].join("/");
    buffer.extend(format!("Content-Length: {}", content.len()).as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(content.as_bytes());
    buffer
}

pub fn get_user_agent(headers: &HashMap<String, String>) -> Vec<u8> {
    let mut buffer = vec![];
    buffer.extend(VERSION.as_bytes());
    buffer.extend(b" ");
    buffer.extend(OK.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(b"Content-Type: text/plain");
    buffer.extend(DELIMITER.as_bytes());
    let user_agent = match headers.get(&"User-Agent".to_string()) {
        Some(v) => v.clone(),
        None => "".to_string(),
    };
    buffer.extend(format!("Content-Length: {}", user_agent.len()).as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(user_agent.as_bytes());
    buffer
}

pub fn get_file(path: &Vec<&str>) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    buffer.extend(VERSION.as_bytes());
    buffer.extend(b" ");
    buffer.extend(OK.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(b"Content-Type: application/octet-stream");
    buffer.extend(DELIMITER.as_bytes());
    let filename = &path[2..].join("/");
    let file = fs::read_to_string(filename)?;
    let file = file.as_bytes();
    buffer.extend(format!("Content-Length: {}", file.len()).as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    buffer.extend(file);
    Ok(buffer)
}

pub fn post_file(path: &Vec<&str>, body: &String) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    buffer.extend(VERSION.as_bytes());
    buffer.extend(b" ");
    buffer.extend(CREATED.as_bytes());
    buffer.extend(DELIMITER.as_bytes());
    let filename = &path[2..].join("/");
    let _ = fs::write(filename, body)?;
    buffer.extend(DELIMITER.as_bytes());
    Ok(buffer)
}
