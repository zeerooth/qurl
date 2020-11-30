use assert_cmd::{Command};
use predicates::{prelude::*};
use httpmock::{MockServer};
use rstest::*;

#[fixture]
fn mock_server_port() -> u16{
    let server = MockServer::start();
    server.mock(|when, then| {
        when.path("/get");
        then.status(200);
    });
    server.port()
}

#[rstest(args, success, text,
    case(vec![format!("http://127.0.0.1:{}/get", mock_server_port()), String::from("-v")], true, predicate::str::contains("HTTP/1.1 200 OK")),
    case(vec![format!("http://127.0.0.1:{}/get", mock_server_port()), String::from("-v")], true, predicate::str::starts_with("Making a request")),
)]
fn test_commands<T: Predicate<str>>(args: Vec<String>, success: bool, text: T) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::cargo_bin("qurl").unwrap().args(args).assert();
    match success {
        true => cmd.success().stdout(text),
        false => cmd.failure().stderr(text)
    };
    Ok(())
}