use crate::handlers::{
    bad_request_response, echo, get_file, get_user_agent, internal_server_error_response,
    not_found_response, ok_response, post_file,
};
use crate::request::{parse_request, Method};
use anyhow::Result;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn processing(stream: TcpStream) -> Result<()> {
    let mut stream = stream;
    let mut buf = vec![0; 2000];
    stream.read(&mut buf)?;

    let request = String::from_utf8(buf)?;
    let request = match parse_request(request) {
        Ok(v) => v,
        Err(_) => {
            match stream.write(&bad_request_response()) {
                Ok(v) => println!("Able write: {} bytes", v),
                Err(e) => println!("Failed to write {:?}", e),
            };
            panic!("Failed to parse request");
        }
    };

    let sub_path: Vec<&str> = request.path.split("/").collect();
    let resp = match (request.method, sub_path[1]) {
        (Method::GET, "") => ok_response(),
        (Method::GET, "echo") => echo(&sub_path),
        (Method::GET, "user-agent") => get_user_agent(&request.headers),
        (Method::GET, "files") => match get_file(&sub_path) {
            Ok(v) => v,
            Err(_) => not_found_response(),
        },
        (Method::POST, "files") => match post_file(&sub_path, &request.body) {
            Ok(v) => v,
            Err(_) => internal_server_error_response(),
        },
        _ => not_found_response(),
    };

    match stream.write(&resp) {
        Ok(v) => println!("Able write: {} bytes", v),
        Err(e) => eprintln!("Failed to write {:?}", e),
    }

    Ok(())
}
