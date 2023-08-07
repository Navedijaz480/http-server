/* Simple HTTP Server */
/* Author : Ramesh Vyas */
// use std::net::TcpListener;
// fn main() {
//     /* Creating a Local TcpListener at Port 8477 */
//     const HOST : &str ="127.0.0.1";
//     const PORT : &str ="8477";
//     /* Concating Host address and Port to Create Final Endpoint */
//     let end_point : String = HOST.to_owned() + ":" +  PORT;
//     /*Creating TCP Listener at our end point */
//     let listener = TcpListener::bind(end_point).unwrap();
//     println!("Web server is listening at port {}",PORT);
//     /* Conneting to any incoming connections */
//     for stream in listener.incoming() {
//         let _stream = stream.unwrap();
//         println!("Connection established!");
//     }
// }    





// /* Simple HTTP Server */
// /* Author : Ramesh Vyas */
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// fn main() {
//     /* Creating a Local TcpListener at Port 8477 */
//     const HOST : &str ="127.0.0.1";
//     const PORT : &str ="8477";
//     /* Concatenating Host address and Port to Create Final Endpoint */
//     let end_point : String = HOST.to_owned() + ":" +  PORT;
//     /*Creating TCP Listener at our end point */
//     let listener = TcpListener::bind(end_point).unwrap();
//     println!("Web server is listening at port {}",PORT);
//     /* Conneting to any incoming connections */
//     for stream in listener.incoming() {
//         let _stream = stream.unwrap();
//         // Call function to process any incomming connections
//         handle_connection(_stream);
//     }
// }
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }




use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_profile = b"GET /profile HTTP/1.1\r\n";
    let get_contact = b"GET /contact HTTP/1.1\r\n";
    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

    } 
    else if buffer.starts_with(get_profile) {
        ("HTTP/1.1 200 OK\r\n\r\n", "profile.html")

    }
    else if buffer.starts_with(get_contact) {
        ("HTTP/1.1 200 OK\r\n\r\n", "contact.html")

    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}











