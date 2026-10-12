use std::io::{Read, Write};
use std::net::TcpListener;

/// Builds the greeting served by the demo app, addressing `name`.
fn greeting(name: &str) -> String {
    format!("Hello {name} from the demo app\n")
}

fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).expect("failed to bind");
    println!("Hello from the demo app — listening on {addr}");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 1024];
                let _ = stream.read(&mut buf);
                let body = greeting("Andre");
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(body.as_bytes());
            }
            Err(_) => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greeting_uses_the_requested_name() {
        assert_eq!(super::greeting("Sam"), "Hello Sam from the demo app\n");
    }
}
