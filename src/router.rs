use std::{collections::HashMap, net::TcpStream, io::{Result, BufReader, BufRead}};

use crate::node::Node;

pub type Handler = fn(TcpStream) -> Result<()>;

#[derive(PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}

pub struct Router {
    routes: HashMap<HttpMethod, Node>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn route_client(&self, client: TcpStream) -> Result<()> {
        let mut reader = BufReader::new(&client);
        let buf = reader.fill_buf()?;

        let mut line = String::new();
        let mut line_reader = BufReader::new(buf);
        let len = line_reader.read_line(&mut line)?;

        reader.consume(len);
        if len == 0 {
            return Ok(());
        }

        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() < 2 {
            Ok(()) // FIX
        } else {
            match (parts[0], parts[1]) {
                ("GET", path) => self.handle(HttpMethod::GET, path, client),
                (_, _) => Ok(())
            }
        }
    }

    pub fn insert(&mut self, method: HttpMethod, path: &str, handler: Handler) {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        node.insert(path, handler);
    }

    pub fn handle(&self, method: HttpMethod, path: &str, client: TcpStream) -> Result<()> {
        if let Some(node) = self.routes.get(&method) {
            if let Some(handler) = node.get(path) {
                return handler(client);
            }
        }
        Ok(())
    }
}