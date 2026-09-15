//! An example GitHub Webhook server demonstrating payload verification with HMAC-SHA256.
//!
//! Run with:
//! ```sh
//! cargo run --example webhook_server
//! ```
//!
//! You can test it by sending a sample webhook payload via curl:
//! ```sh
//! curl -X POST http://127.0.0.1:3000/webhook \
//!   -H "Content-Type: application/json" \
//!   -H "X-GitHub-Event: ping" \
//!   -H "X-Hub-Signature-256: sha256=f101961424ae12c9b9d55cd4e562f4040c27ff1b9a8b7d2c4ba5cfc15e62d54f" \
//!   -d '{"zen":"Design for failure.","hook_id":423885699,"hook":{"type":"App","id":423885699,"name":"web","active":true,"events":["issues"],"config":{"content_type":"json","insecure_ssl":"0","url":"https://smee.io/R"},"updated_at":"2023-07-13T09:30:45Z","created_at":"2023-07-13T09:30:45Z","app_id":360617,"deliveries_url":"https://api.github.com/app/hook/deliveries"}}'
//! ```

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use octocrab::models::webhook_events::{
    WebhookEvent, WebhookEventError, WebhookEventType, WebhookVerificationError, WebhookVerifier,
};

fn handle_client(mut stream: TcpStream, verifier: &WebhookVerifier) {
    let mut buffer = Vec::new();
    let mut temp = [0u8; 1024];

    // Read headers and body from HTTP stream
    let (headers, body_offset) = loop {
        let n = match stream.read(&mut temp) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };
        buffer.extend_from_slice(&temp[..n]);

        if let Some(pos) = buffer.windows(4).position(|w| w == b"\r\n\r\n") {
            let header_str = String::from_utf8_lossy(&buffer[..pos]);
            break (header_str.to_string(), pos + 4);
        }
    };

    // Parse Content-Length to ensure full body is read
    let content_length: usize = headers
        .lines()
        .find_map(|line| {
            let line = line.to_lowercase();
            if line.starts_with("content-length:") {
                line.split(':').nth(1)?.trim().parse().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);

    while buffer.len() < body_offset + content_length {
        let n = match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };
        buffer.extend_from_slice(&temp[..n]);
    }

    let body = &buffer[body_offset..body_offset + content_length];

    // Extract GitHub webhook headers
    let event_type = headers.lines().find_map(|line| {
        if line.to_lowercase().starts_with("x-github-event:") {
            Some(line.split(':').nth(1)?.trim())
        } else {
            None
        }
    });

    let signature = headers.lines().find_map(|line| {
        if line.to_lowercase().starts_with("x-hub-signature-256:") {
            Some(line.split(':').nth(1)?.trim())
        } else {
            None
        }
    });

    let (event_type, signature) = match (event_type, signature) {
        (Some(e), Some(s)) => (e, s),
        (None, _) => {
            let response =
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 26\r\n\r\nMissing X-GitHub-Event";
            let _ = stream.write_all(response.as_bytes());
            return;
        }
        (_, None) => {
            let response =
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 33\r\n\r\nMissing X-Hub-Signature-256";
            let _ = stream.write_all(response.as_bytes());
            return;
        }
    };

    // Verify signature and parse the payload
    match verifier.verify_and_parse(event_type, signature, body) {
        Ok(event) => {
            println!(
                "Successfully verified signature for event: {:?}",
                event.kind
            );
            handle_webhook_event(event);

            let response = "HTTP/1.1 200 OK\r\nContent-Length: 16\r\n\r\nWebhook received";
            let _ = stream.write_all(response.as_bytes());
        }
        Err(WebhookEventError::Verification {
            source: WebhookVerificationError::DeprecatedSha1,
        }) => {
            eprintln!("Rejected delivery: SHA-1 signatures are deprecated by GitHub");
            let response =
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 35\r\n\r\nSHA-1 signatures are deprecated";
            let _ = stream.write_all(response.as_bytes());
        }
        Err(WebhookEventError::Verification { source }) => {
            eprintln!("Signature verification failed: {}", source);
            let response =
                "HTTP/1.1 401 Unauthorized\r\nContent-Length: 29\r\n\r\nInvalid webhook signature";
            let _ = stream.write_all(response.as_bytes());
        }
        Err(WebhookEventError::Json { source }) => {
            eprintln!("JSON deserialization failed: {}", source);
            let response =
                "HTTP/1.1 400 Bad Request\r\nContent-Length: 25\r\n\r\nMalformed JSON payload";
            let _ = stream.write_all(response.as_bytes());
        }
    }
}

fn handle_webhook_event(event: WebhookEvent) {
    match event.kind {
        WebhookEventType::Ping => {
            println!("-> Received Ping event from GitHub!");
        }
        WebhookEventType::Issues => {
            println!("-> Received Issues event!");
        }
        WebhookEventType::PullRequest => {
            println!("-> Received PullRequest event!");
        }
        WebhookEventType::Push => {
            println!("-> Received Push event!");
        }
        other => {
            println!("-> Received event: {:?}", other);
        }
    }
}

fn main() {
    let secret = std::env::var("WEBHOOK_SECRET").unwrap_or_else(|_| "secret".to_string());
    let verifier = WebhookVerifier::new(&secret);

    let address = "127.0.0.1:3000";
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", address, e);
            return;
        }
    };

    println!("GitHub Webhook Server listening on http://{}", address);
    println!("Webhook secret: '{}'", secret);
    println!("Awaiting GitHub webhook deliveries...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream, &verifier),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
