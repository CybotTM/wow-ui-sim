use super::{LuaCommand, Response};
use std::io::{BufRead, BufReader, Write};
use std::net::Shutdown;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn screenshot_client_forwards_png_choice() {
    let directory = tempfile::tempdir().unwrap();
    let socket_path = directory.path().join("client.sock");
    let listener = UnixListener::bind(&socket_path).unwrap();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    thread::scope(|scope| {
        let server = scope.spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            super::handle_connection(stream, &cmd_tx).unwrap();
        });
        let client = scope.spawn(|| {
            super::client::screenshot(
                &socket_path,
                "client.png",
                23,
                17,
                Some("Panel".into()),
                None,
                true,
            )
        });
        let command = cmd_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let LuaCommand::Screenshot {
            output,
            width,
            height,
            filter,
            crop,
            png,
            respond,
        } = command
        else {
            panic!("expected screenshot command");
        };
        assert_eq!(output, "client.png");
        assert_eq!((width, height), (23, 17));
        assert_eq!(filter.as_deref(), Some("Panel"));
        assert_eq!(crop, None);
        assert!(png);
        respond
            .send(Response::Output("saved client.png".into()))
            .unwrap();
        assert_eq!(client.join().unwrap().unwrap(), "saved client.png");
        server.join().unwrap();
    });
}

#[test]
fn screenshot_socket_preserves_png_choice_and_defaults_omitted_to_false() {
    const IO_TIMEOUT: Duration = Duration::from_secs(5);
    let directory = tempfile::tempdir().unwrap();
    let socket_path = directory.path().join("screenshot.sock");
    let listener = UnixListener::bind(&socket_path).unwrap();
    let client_stream = UnixStream::connect(&socket_path).unwrap();
    let (server_stream, _) = listener.accept().unwrap();
    for stream in [&client_stream, &server_stream] {
        stream.set_read_timeout(Some(IO_TIMEOUT)).unwrap();
        stream.set_write_timeout(Some(IO_TIMEOUT)).unwrap();
    }
    let mut reader = BufReader::new(client_stream);
    let (cmd_tx, cmd_rx) = mpsc::channel();
    thread::scope(|scope| {
        let server = scope.spawn(move || super::handle_connection(server_stream, &cmd_tx));
        for expected_png in [true, false] {
            let mut payload = serde_json::json!({
                "Screenshot": {
                    "output": "socket capture.input", "width": 913, "height": 517,
                    "filter": "TestPanel", "crop": "37x23+4+8"
                }
            });
            if expected_png {
                payload["Screenshot"]["png"] = serde_json::json!(true);
            }
            writeln!(reader.get_mut(), "{payload}").unwrap();
            let command = cmd_rx.recv_timeout(IO_TIMEOUT).unwrap();
            let LuaCommand::Screenshot {
                output,
                width,
                height,
                filter,
                crop,
                png,
                respond,
            } = command
            else {
                panic!("expected screenshot command");
            };
            assert_eq!(output, "socket capture.input");
            assert_eq!((width, height), (913, 517));
            assert_eq!(filter.as_deref(), Some("TestPanel"));
            assert_eq!(crop.as_deref(), Some("37x23+4+8"));
            assert_eq!(png, expected_png);
            let expected_body = format!("PNG: {png}\nsecond line: 雪");
            respond
                .send(Response::Output(expected_body.clone()))
                .unwrap();
            let mut line = String::new();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            assert!(line.ends_with('\n'));
            match serde_json::from_str::<Response>(&line).unwrap() {
                Response::Output(body) => assert_eq!(body, expected_body),
                other => panic!("expected output, got {other:?}"),
            }
        }
        reader.get_ref().shutdown(Shutdown::Both).unwrap();
        server.join().unwrap().unwrap();
    });
}
