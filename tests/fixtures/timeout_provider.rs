use std::io::{Read, Write};
use std::time::Duration;

/// Simulates either CLI with controllable stdout/stderr activity and delayed structured output.
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
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
