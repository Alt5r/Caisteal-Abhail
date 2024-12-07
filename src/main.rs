use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::BufReader;
use rand::Rng;

//. using the pages, which are function in the pages.rs which are then pattern matched in handle client function here
mod pages;
use pages::*;

mod helpers;
use helpers::*;

/*
example headers
["GET /test HTTP/1.1", "Host: 127.0.0.1:8080", "Connection: keep-alive", "sec-ch-ua: \"Brave\";v=\"107\", \"Chromium\";v=\"107\", \"Not=A?Brand\";v=\"24\"", "sec-ch-ua-mobile: ?0", "sec-ch-ua-platform: \"Linux\"", "Upgrade-Insecure-Requests: 1", "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36", "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*
q=0.8", "Sec-GPC: 1", "Accept-Language: en-GB,en", "Sec-Fetch-Site: none", "Sec-Fetch-Mode: navigate", "Sec-Fetch-User: ?1", "Sec-Fetch-Dest: document", "Accept-Encoding: gzip, deflate, br"]

*/

fn handle_client(mut stream: TcpStream, addr:SocketAddr) {
    let mut buffer = BufReader::new(&stream);

    let request: Vec<String> = buffer
        .lines()
        .filter_map(|result| result.ok())
        .take_while(|line| !line.is_empty())
        .collect();

    if !request.is_empty() {
        let endpoint = root(request.clone());
        let response = match endpoint.as_str() {
            "/test" => test(request, addr),
            "/qry-p-test" => qrstr(request),
            _ => notFound(),
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send response: {}", e);
        }
    }
}



// pages on the web server 



///  stripping the method and the http version to get the directory requested
fn root(response:Vec<String>) -> String {
    
    let directory = response[0].strip_prefix("GET ").unwrap_or(&response[0]);
    let directory = directory.strip_suffix(" HTTP/1.1").unwrap_or(directory);
    String::from(directory)

    // check if has query string parameters
}

// entry point

fn main() {

    // tcp listener

    let listener = TcpListener::bind("0.0.0.0:8080").expect("failed to bind, port maybe in use");

    println!("server listing on addressp provided");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut addr: SocketAddr;   
                if let Ok(addr) = stream.peer_addr() {
                    println!("{}", &addr);
                    std::thread::spawn(move || handle_client(stream, addr));
                }
                
            }
            Err(e) => {eprintln!("{}", e)}
        } 
    }
}