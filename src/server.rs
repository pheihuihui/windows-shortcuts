use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    alert,
    constants::APP_CONFIG,
    utils::inputs::switch_windows,
    utils::other_functions::{capture_screen, switch_to_monitor, switch_to_tv},
};

pub struct ShortServer {
    listener: Arc<TcpListener>,
}

impl ShortServer {
    pub fn from_config() -> Self {
        let port = APP_CONFIG.server_port.to_owned();
        let url = format!("0.0.0.0:{port}");
        ShortServer {
            listener: Arc::new(TcpListener::bind(url).unwrap()),
        }
    }

    pub fn start_server(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(val) => self.handle_connection(val),
                Err(_) => {}
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        let urls: Vec<_> = http_request
            .clone()
            .into_iter()
            .filter(|x| x.starts_with("HEAD") || x.starts_with("GET"))
            .collect();
        if urls.len() == 1 {
            let url = &urls[0];
            let url: Vec<_> = url.split(" ").collect();
            if url.len() >= 2 {
                let url = url[1];
                match url {
                    "/switch_to_tv" => switch_to_tv(),
                    "/switch_to_monitor" => switch_to_monitor(),
                    "/switch_windows" => switch_windows(),
                    "/capture_screen" => capture_screen(),
                    "/hello" => alert!("Hello"),
                    _ => {}
                }
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}

impl Default for ShortServer {
    fn default() -> ShortServer {
        let listener = TcpListener::bind("0.0.0.0:9111").unwrap();
        let val: ShortServer = ShortServer {
            listener: Arc::new(listener),
        };
        val
    }
}
