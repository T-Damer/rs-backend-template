use std::{
    fs::read_to_string,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let server_address = env!("SERVER_ADDRESS");
    let listener = TcpListener::bind(server_address).unwrap();

    println!("Server is listening on {}", server_address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let target_folder = "res/";

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", format!("{target_folder}/hello.html"))
    } else {
        (
            "HTTP/1.1 404 NOT FOUND",
            format!("{target_folder}/404.html"),
        )
    };

    let contents = read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
