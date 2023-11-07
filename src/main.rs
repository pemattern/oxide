use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

struct OxideApp {
    addr: String,
}

impl OxideApp {
    fn new(addr: &str) -> OxideApp {
        let app = OxideApp { addr: String::from(addr) };
        app
    }

    fn listen(&mut self, port: u16) {
        let full_address = format!("{}{}{}", self.addr, ":", &port.to_string());
        let listener = TcpListener::bind(&full_address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            Self::handle_connection(stream);
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}


fn main() {
    let mut app = OxideApp::new("127.0.0.1");
    app.listen(3000);
}


