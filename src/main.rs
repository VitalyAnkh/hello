extern crate hello;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool=ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { handle_connection(stream) });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));
    println!("***********************");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "h.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "h.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };


    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


//fn calculate_length(s: &mut String) -> usize {
//    s.push_str("hahahhahhahhahhahhah");
//    (*s).len()
//}
//
//fn call() {
//    let mut s1 = String::from("Hello");
//    let s2 = &mut s1;
////    println!("The length of '{}' is {}",*s2, calculate_length(s2));
////    will raise the problem "two mutable reference"
//    modify(&mut s1);
//    println!("After modified, s1 is '{}'.", s1);
////    let reference_to_nothing = dangle();
//}
//
//fn modify(some_string: &mut String) {
//    (*some_string).push_str(", world!");
//}
