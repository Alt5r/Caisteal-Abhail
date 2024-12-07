use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::BufReader;
use rand::Rng;

use crate::ReqT;

/// extracts any query string parameters, request in this case will be GET part of it 

pub fn type_req(request:String) -> ReqT {
    if request.contains("GET") {
        return ReqT::GET
    } else if request.contains("POST") {
        return ReqT::POST
        
    } else if request.contains("PUT") {
        return ReqT::PUT
    } else {
        return ReqT::DELETE
    }


}

pub fn qry_str(request:String) -> Vec<String> {
    
    if request.contains("?") && !request.contains("&") {
        // means that it contains query parameters and only one 
        let index: Option<usize> = request.find("?");
        let index = index.unwrap();
        let parameter_val = &request[index + 1..];

        let parameter_val:Vec<String> = parameter_val.split("=").map(String::from).collect();

        println!("{:?}", parameter_val);
       
        parameter_val
    } else {
        Vec::new()
    }
}

/* 

pub fn hret(html:&str) -> String {
    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html.as_str()).as_str());

    response
}

*/