use std::fs;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::collections::HashMap;

mod envs;
mod lib;

use walkdir::WalkDir;

type HTMLContents = HashMap<String, String>;

const LEN_OF_EXT: usize = 5;

fn main() {
    let host: String = envs::server_host();
    let listener = TcpListener::bind(&host).unwrap();

    println!("Connection established!");
    println!("Listening on {}", host);

    let pool = lib::ThreadPool::new(envs::thread_pool().parse().unwrap());

    let mut contents: HTMLContents = HashMap::new();
    let html_path = envs::html_folder();

    for path in find_all_html(&html_path).into_iter() {
        contents.insert(path[html_path.len()..(path.len() - LEN_OF_EXT)].to_owned(), fs::read_to_string(path).unwrap());
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        let content = contents.clone();

        pool.execute(move || { handle_connection(stream, content ) });
    }

    println!("Shutting down");
}

fn find_all_html(path: &String) -> Vec<String> {
    WalkDir::new(&path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| {
                match e {
                    Ok (entry) => {
                        let es = entry.path().to_str().unwrap().to_string();
                        if es.ends_with(".html") {
                            Some(es)
                        } else {
                            None
                        } 
                    },
                    _ => None,
                }
            })
            .collect()
}

fn handle_connection(mut stream: TcpStream, contents: HTMLContents) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response =
    if let Some((url,content)) = contents.iter().find(|(k, _)| buffer.starts_with(format!("GET {} HTTP/1.1\r\n" ,k).as_bytes())) {
        println!("Processing response for {}", url);
        ("HTTP/1.1 200 OK", content)
    } else {
        println!("Nothing found for request");
        ("HTTP/1.1 404 NOT FOUND", &contents["/404"])
        
    };
    stream.write(format!("{}\r\nContent-Length: {}\r\n\r\n{}", response.0, response.1.len(), response.1).as_bytes()).unwrap();
}
