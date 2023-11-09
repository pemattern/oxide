use crate::router::{Router, HttpMethod};
use std::net::TcpStream;
use std::io::Result;

pub fn configure(router: &mut Router) {
    router.insert(HttpMethod::GET, "/", index);
}

fn index(client: TcpStream) -> Result<()> {
    Ok(())
}