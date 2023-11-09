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
}


fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
        
    let mut router = Router::new();
    routes::configure(&mut router);
    for client in listener.incoming() {
        router.route_client(client.unwrap())?;
    }
    Ok(())
}


