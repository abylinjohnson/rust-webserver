use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
fn main() {
    let listener: TcpListener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("Connection Established");
        handle_connection(stream);
    }

}

fn handle_connection(mut stream : TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Requst: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let about = b"GET /about HTTP/1.1\r\n";
    let contact = b"GET /contact HTTP/1.1\r\n";
    let menu = b"GET /menu HTTP/1.1\r\n";
    let reviews = b"GET /reviews HTTP/1.1\r\n";
    let (status_line, filename) = 
    if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "src/website/index.html")
    } else if buffer.starts_with(about){
        ("HTTP/1.1 200 OK", "src/website/about.html")
    } else if buffer.starts_with(contact){
        ("HTTP/1.1 200 OK", "src/website/contact.html")
    } else if buffer.starts_with(menu){
        ("HTTP/1.1 200 OK", "src/website/menu.html")
    } else if buffer.starts_with(reviews){
        ("HTTP/1.1 200 OK", "src/website/reviews.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "src/website/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush();
    
}
