mod router;
mod routes;
mod node;

use std::{
    io::{prelude::*, Result},
    net::{TcpListener, TcpStream},
};

use router::Router;
struct OxideApp {
    addr: String,
}

impl OxideApp {
    fn new(addr: &str) -> Self {
        let app = OxideApp { addr: String::from(addr) };
        app
    }

    fn listen(&self, port: u16) {
        let full_address = format!("{}{}{}", self.addr, ":", &port.to_string());
        let listener = TcpListener::bind(&full_address).unwrap();
        
        let mut router = Router::new();
        routes::configure(&mut router);

        for client in listener.incoming() {
            router.route_client(client)?;
        }
    }
}


fn main() {
    let app = OxideApp::new("127.0.0.1");
    app.listen(3000);
}


