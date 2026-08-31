use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

/// Simulates either the Codex CLI or an OpenCode server with controllable activity and delay.
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == "serve") {
        serve_opencode();
        return;
    }
    let codex = args.iter().any(|arg| arg == "--output-schema");
    if codex {
        let mut prompt = String::new();
        std::io::stdin().read_to_string(&mut prompt).unwrap();
    }
    let mode = std::fs::read_to_string("mode").unwrap();
    for _ in 0..12 {
        match mode.as_str() {
            "stdout" => {
                let event = if codex { "turn.started" } else { "step_start" };
                println!("{{\"type\":\"{event}\"}}");
                std::io::stdout().flush().unwrap();
            }
            "stderr" => {
                eprintln!("fixture progress");
                std::io::stderr().flush().unwrap();
            }
            _ => {}
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let response = std::fs::read_to_string("response").unwrap();
    if codex {
        let output = args
            .windows(2)
            .find(|pair| pair[0] == "--output-last-message")
            .unwrap();
        std::fs::write(&output[1], response).unwrap();
    } else {
        println!("{response}");
    }
}

/// Runs a minimal loopback OpenCode-compatible server for timeout contract tests.
fn serve_opencode() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    println!("opencode server listening on http://{address}");
    std::io::stdout().flush().unwrap();
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(|| handle_opencode_connection(stream));
        }
    }
}

/// Handles one bounded HTTP request used by the OpenCode adapter's session lifecycle.
fn handle_opencode_connection(mut stream: TcpStream) {
    let request = match read_http_request(&mut stream) {
        Some(request) => request,
        None => return,
    };
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or_default();
    let mode = std::fs::read_to_string("mode").unwrap();
    if path == "/global/health" {
        write_json(&mut stream, r#"{"healthy":true,"version":"fixture"}"#);
    } else if path == "/session" {
        write_json(&mut stream, r#"{"id":"session-1"}"#);
    } else if path == "/event" {
        stream
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: keep-alive\r\n\r\n",
            )
            .unwrap();
        stream.flush().unwrap();
        if mode.trim() != "silent" {
            loop {
                if stream
                    .write_all(b"data: {\"type\":\"session.status\",\"properties\":{\"sessionID\":\"session-1\"}}\n\n")
                    .is_err()
                {
                    return;
                }
                if stream.flush().is_err() {
                    return;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    } else if path.ends_with("/message") {
        if mode.trim() != "silent" {
            for _ in 0..12 {
                std::thread::sleep(Duration::from_millis(100));
            }
        } else {
            std::thread::sleep(Duration::from_secs(2));
        }
        let response = std::fs::read_to_string("response").unwrap();
        let body = format!(r#"{{"info":{{"structured_output":{response}}},"parts":[]}}"#);
        write_json(&mut stream, &body);
    } else if path.ends_with("/abort") || (path.starts_with("/session/") && path != "/session/session-1") {
        write_json(&mut stream, "true");
    } else {
        write_json(&mut stream, "{}");
    }
}

/// Reads one HTTP request through its declared content length without waiting for connection close.
fn read_http_request(stream: &mut TcpStream) -> Option<String> {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 4096];
    let header_end;
    loop {
        let count = stream.read(&mut buffer).ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if let Some(position) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
        if bytes.len() > 64 * 1024 {
            return None;
        }
    }
    let headers = String::from_utf8_lossy(&bytes[..header_end]);
    let content_length = headers
        .lines()
        .filter_map(|line| line.split_once(':'))
        .find_map(|(name, value)| {
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);
    while bytes.len() < header_end + content_length {
        let count = stream.read(&mut buffer).ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if bytes.len() > 4 * 1024 * 1024 {
            return None;
        }
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// Writes a small JSON response and closes the fixture connection.
fn write_json(stream: &mut TcpStream, body: &str) {
    let body = body.as_bytes();
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes()).unwrap();
    stream.write_all(body).unwrap();
    stream.flush().unwrap();
}
