use reqwest::Url;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn get_params_from_localhost_request(port: u16, path: String) {
    let listener = TcpListener::bind(format!("localhost:{}", port)).unwrap();
    handle_connection(listener.incoming().next().unwrap().unwrap())
}

fn handle_connection(mut stream: TcpStream) {
    let request = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .next()
        .unwrap();

    println!("Request: {:#?}", request);
    print_connection(request);
    write_close_tab_msg(stream);
}

fn write_close_tab_msg(mut stream: TcpStream) {
    let content = "<h1>You can close this tab now</h1>";
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn print_connection(raw: String) {
    let raw_url: Vec<&str> = raw.as_str().split_whitespace().collect();
    println!("{:#?}", raw_url);
    let url = Url::try_from(format!("http://example.com{}", raw_url[1]).as_str()).unwrap();
    let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
    println!("Code: {:?}", params.get("code"))
    // for (param, value) in params {
    //     println!("Param: {}, Value: {}", param, value);
    // }
}
