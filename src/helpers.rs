use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::BufReader;
use rand::Rng;

/// extracts any query string parameters, request in this case will be GET part of it 

pub fn type_req(request:String) -> String {
    if request.contains("GET") {
        return String::from("GET")
    } else if request.contains("POST") {
        return String::from("POST")
        
    } else if request.contains("PUT") {
        return String::from("PUT")
    } else {
        return String::from("DELETE")
    }


}

pub fn qry_str(request:String) -> String {
    
    if request.contains("?") && !request.contains("&"){
        // means that it contains query parameters and only one 
        let index: Option<usize> = request.find("?");
        let index = index.unwrap();
        let parameter_val = &request[index + 1..];

        let parameter_val = parameter_val.split("=").map(String::from).collect();


        parameter_val
        
    } else {
        // somethign here
        String::from("Here")
    }

}