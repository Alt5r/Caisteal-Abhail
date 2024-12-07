use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::BufReader;
use rand::Rng;

use crate::helpers::{self, qry_str};

pub fn notFound() -> String {
    let html = "<h1>404 Not Found</h1>";
    format!(
        "HTTP/1.1 404 NOT FOUND\r\n\
         Content-Type: text/html\r\n\
         Content-Length: {len}\r\n\
         \r\n\
         {h}",
        len = html.len(),
        h = html
    )
}


pub fn test(request:Vec<String>, addr:SocketAddr) -> String {
    let mut rng = rand::thread_rng();
    let option = rng.gen_range(0..3);
    let mut pages = Vec::new();

    let html1 = format!("<h1>Hello there {} </h1>", addr);
    let html2 = format!("<h1>Different html page with randomness</h1>");
    let html3 = format!("<h1>Goodbye</h1>");

    pages.push(html1);
    pages.push(html2);
    pages.push(html3);

    let html = &pages[option];
    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html.as_str()).as_str());

    // do something for a specific endpoint here

    println!("hit the /test endpoint from {:?}", request[1].strip_prefix("Host: ").unwrap());
    response
}

pub fn qrstr(request:Vec<String>) -> String {
    // returns list of strings where it alternates key, pair value throuhgout the list

    let stripping:Vec<String> = request[0].split(" ").map(String::from).collect();
    let stripping = &stripping[1];
    println!("{}", stripping);
    
    let result = qry_str(stripping.clone());
    //result
    String::from("hello")
}