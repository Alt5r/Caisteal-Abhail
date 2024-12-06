use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

/*
example headers
["GET /test HTTP/1.1", "Host: 127.0.0.1:8080", "Connection: keep-alive", "sec-ch-ua: \"Brave\";v=\"107\", \"Chromium\";v=\"107\", \"Not=A?Brand\";v=\"24\"", "sec-ch-ua-mobile: ?0", "sec-ch-ua-platform: \"Linux\"", "Upgrade-Insecure-Requests: 1", "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/107.0.0.0 Safari/537.36", "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*
q=0.8", "Sec-GPC: 1", "Accept-Language: en-GB,en", "Sec-Fetch-Site: none", "Sec-Fetch-Mode: navigate", "Sec-Fetch-User: ?1", "Sec-Fetch-Dest: document", "Accept-Encoding: gzip, deflate, br"]

*/

fn handle_client(mut stream: TcpStream) {
    let mut buffer = BufReader::new(&stream);

    let request: Vec<String> = buffer
        .lines()
        .filter_map(|result| result.ok())
        .take_while(|line| !line.is_empty())
        .collect();

    if !request.is_empty() {
        let endpoint = root(request.clone());
        let response = match endpoint.as_str() {
            "/test" => test(request),
            _ => notFound(),
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send response: {}", e);
        }
    }
}



// pages on the web server 


fn notFound() -> String {
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


fn test(request:Vec<String>) -> String {

    let html = "<h1>Hello there in html header </h1>";

    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html).as_str());

    // do something for a specific endpoint here

    println!("hit the /test endpoint from {:?}", request[1].strip_prefix("Host: ").unwrap());
    response
}
fn root(response:Vec<String>) -> String {
    // stripping the method and the http version to get the directory requested
    let directory = response[0].strip_prefix("GET ").unwrap_or(&response[0]);
    let directory = directory.strip_suffix(" HTTP/1.1").unwrap_or(directory);
    String::from(directory)
}
// entry point

fn main() {

    // tcp listener

    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind, port maybe in use");

    println!("server listing on addressp provided");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {eprintln!("{}", e)}
        } 
    }
}