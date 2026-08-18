use std::net::TcpStream; pub fn is_ollama_running() -> bool { TcpStream::connect("127.0.0.1:11434").is_ok() }
