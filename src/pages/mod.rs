use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::BufReader;
use rand::Rng;

use mysql::*;
use mysql::prelude::*;

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

/// returns list of strings where it alternates key, pair value throuhgout the list
pub fn qrstr(request:Vec<String>) -> String {
    
    // strips the method and http version from the request paramter
    let stripping:Vec<String> = request[0].split(" ").map(String::from).collect();
    let stripping = &stripping[1];

    println!("{}", stripping); // for debugging
    
    let result = qry_str(stripping.clone());
    //result
    let html  = format!("<h1>your paramters are {:?} </h1>", result);

    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html.as_str()).as_str());

    response

}
/// gets the users of a sql database
pub fn getUsers(request:Vec<String>) -> Result<String, mysql::Error> {

    let url = "mysql://root:passwordtest@192.168.1.128:32768/test";

    // Create a connection pool
    let pool = Pool::new(url)?;

    // Get a connection from the pool
    let mut conn = pool.get_conn()?;

    let result:Vec<(String, String)> = conn.query("SELECT User, Password FROM `test-table`").unwrap();

    let html  = format!("<h1>your paramters are {:?} </h1>", result);

    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html.as_str()).as_str());

    Ok(response)
}
/// adds a user to a sql database
pub fn addUser(request:Vec<String>) -> Result<String, mysql::Error> {

    // strips the method and http version from the request paramter
    let stripping:Vec<String> = request[0].split(" ").map(String::from).collect();
    let stripping = &stripping[1];

    println!("{}", stripping); // for debugging
    
    let result = qry_str(stripping.clone());

    // result at this point contains the username and the password

    let creds:(String, String) = (result[1].clone(), result[3].clone());

    let url = "mysql://root:passwordtest@192.168.1.128:32768/test";

    // Create a connection pool
    let pool = Pool::new(url)?;

    // Get a connection from the pool
    let mut conn = pool.get_conn()?;

    //let result:Vec<(u32, String, String)> = conn.query("SELECT P_KEY, User, Password FROM `test-table`").unwrap();

    conn.exec_drop(
        r"INSERT INTO `test-table` (User, Password) VALUES (:user, :password)",
        params! {
            "user" => creds.0,
            "password" => creds.1,
        },
    )?;

    let html  = format!("<h1>Successfully added user </h1>");

    let response = String::from(format!("HTTP/1.1 200 OK\r\n\
    Content-Type: text/html\r\n\
    Content-Length: {len}\r\n\
    \r\n\
    {h}", len=html.len(), h=html.as_str()).as_str());

    Ok(response)

}