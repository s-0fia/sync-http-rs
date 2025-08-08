use request::Request;
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpListener},
};

pub mod errors;
pub mod mime;
pub mod request;
pub mod server;
