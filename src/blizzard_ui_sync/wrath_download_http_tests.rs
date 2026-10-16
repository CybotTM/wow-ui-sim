//! Bounded localhost coverage of the actual archive HTTP retry loop.
#![cfg(feature = "casc")]

use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

const SERVER_LIFETIME: Duration = Duration::from_secs(10);
const SOCKET_TIMEOUT: Duration = Duration::from_millis(250);
const REQUEST_LIFETIME: Duration = Duration::from_secs(1);
const ACCEPT_INTERVAL: Duration = Duration::from_millis(5);
const MAX_REQUEST_BYTES: usize = 16 * 1024;
const ARCHIVE_BYTES: &[u8] = b"\x00\x1f\x8b\xffarchive\r\n\x00";

#[derive(Clone, Copy)]
struct Reply {
    status: &'static str,
    retry_after: Option<&'static str>,
    body: &'static [u8],
}

const OK: Reply = Reply {
    status: "200 OK",
    retry_after: None,
    body: ARCHIVE_BYTES,
};
const UNAVAILABLE: Reply = Reply {
    status: "503 Service Unavailable",
    retry_after: Some("0"),
    body: b"temporarily unavailable",
};
const NOT_FOUND: Reply = Reply {
    status: "404 Not Found",
    retry_after: None,
    body: b"not found",
};

#[test]
fn fetch_archive_http_200_preserves_exact_bytes() {
    let (result, requests) = fetch_from_local_server(&[OK]);
    assert_eq!(result.expect("HTTP 200 download"), ARCHIVE_BYTES);
    assert_eq!(requests, 1);
}

#[test]
fn fetch_archive_retries_503_with_retry_after_zero_then_returns_200() {
    let (result, requests) = fetch_from_local_server(&[UNAVAILABLE, OK]);
    assert_eq!(result.expect("download after transient 503"), ARCHIVE_BYTES);
    assert_eq!(requests, 2, "one retry must reach the HTTP server");
}

#[test]
fn fetch_archive_does_not_retry_permanent_404() {
    let (result, requests) = fetch_from_local_server(&[NOT_FOUND]);
    let message = result.expect_err("HTTP 404 must fail").to_string();
    assert!(message.contains("404"), "expected status error: {message}");
    assert_eq!(requests, 1, "permanent errors must not retry");
}

#[test]
fn fetch_archive_exhausts_after_three_503_responses() {
    let (result, requests) = fetch_from_local_server(&[UNAVAILABLE; 3]);
    let message = result
        .expect_err("three HTTP 503 replies must fail")
        .to_string();
    assert!(message.contains("503"), "expected status error: {message}");
    assert_eq!(
        requests, 3,
        "three total attempts must reach the HTTP server"
    );
}

fn fetch_from_local_server(replies: &[Reply]) -> (std::io::Result<Vec<u8>>, usize) {
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
        .expect("bind ephemeral IPv4 loopback fixture");
    listener.set_nonblocking(true).expect("nonblocking accept");
    let address = listener.local_addr().expect("fixture address");
    let url = format!("http://{address}/archive.tar.gz");
    let stop = AtomicBool::new(false);
    thread::scope(|scope| {
        let server = scope.spawn(|| serve_replies(listener, replies, &stop));
        let result = super::fetch_archive(&url);
        stop.store(true, Ordering::Release);
        let requests = server
            .join()
            .expect("fixture thread panicked")
            .expect("HTTP fixture failed");
        (result, requests)
    })
}

// The scoped thread is also joined during unwinding. Nonblocking accept,
// finite socket timeouts and absolute deadlines bound that cleanup even if
// fetch_archive panics. Keep accepting until the client returns, so an extra
// retry is observable rather than silently lost after the expected replies.
fn serve_replies(listener: TcpListener, replies: &[Reply], stop: &AtomicBool) -> io::Result<usize> {
    let deadline = Instant::now() + SERVER_LIFETIME;
    let mut requests = 0;
    while !stop.load(Ordering::Acquire) && Instant::now() < deadline {
        let mut stream = match listener.accept() {
            Ok((stream, _)) => stream,
            Err(cause) if cause.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(ACCEPT_INTERVAL);
                continue;
            }
            Err(cause) => return Err(cause),
        };
        stream.set_read_timeout(Some(SOCKET_TIMEOUT))?;
        stream.set_write_timeout(Some(SOCKET_TIMEOUT))?;
        read_request_headers(&mut stream, deadline)?;
        // An unexpected retry gets a complete terminal success, not another
        // retryable status. Its request is still counted and fails the test.
        let reply = replies.get(requests).copied().unwrap_or(OK);
        requests += 1;
        write_reply(&mut stream, reply)?;
    }
    Ok(requests)
}

fn read_request_headers(stream: &mut TcpStream, server_deadline: Instant) -> io::Result<()> {
    let deadline = server_deadline.min(Instant::now() + REQUEST_LIFETIME);
    let mut request = Vec::new();
    let mut buffer = [0; 1024];
    while Instant::now() < deadline && request.len() < MAX_REQUEST_BYTES {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incomplete HTTP request",
            ));
        }
        request.extend_from_slice(&buffer[..count]);
        if request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            return Ok(());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "HTTP request exceeded fixture bounds",
    ))
}

fn write_reply(stream: &mut TcpStream, reply: Reply) -> io::Result<()> {
    let mut headers = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nConnection: close\r\n",
        reply.status,
        reply.body.len(),
    );
    if let Some(value) = reply.retry_after {
        headers.push_str(&format!("Retry-After: {value}\r\n"));
    }
    headers.push_str("\r\n");
    stream.write_all(headers.as_bytes())?;
    stream.write_all(reply.body)
}
