use assert_cmd::{Command};
use predicates::{prelude::*};
use httpmock::{MockServer};
use httpmock::Method;
use rstest::*;
use std::fs;

fn test_json_response() -> String {
    fs::read_to_string("tests/resources/mock.json").expect("reading mock json failed")
}

fn mock_server_port() -> u16 {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(Method::GET)
            .path("/get");
        then.status(200).header("Content-Type", "application/json").body(test_json_response());
    });
    server.mock(|when, then| {
        when.method(Method::POST)
            .path("/post")
            .header("Authorization", "Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
        then.status(200).header("Content-Type", "text/html").body("success");
    });
    server.mock(|when, then| {
        when.method(Method::PUT)
            .path("/put")
            .header("Authorization", "Bearer AaBbCcDdEeFfGg0987654321fFeEdDcCbBaA");
        then.status(200).header("Content-Type", "text/html").body("success");
    });
    server.mock(|when, then|{
        when.path("/redirect");
        then.temporary_redirect(format!("/redirect2"));
    });
    server.mock(|when, then|{
        when.path("/redirect2");
        then.status(200).header("Content-Type", "text/html").body("success");
    });
    server.mock(|when, then| {
        when.method(Method::PATCH)
            .path("/patch")
            .body("Some spaced body text .,?!$%&#*");
        then.status(200).header("Content-Type", "text/html").body("success");
    });
    server.port()
}

#[rstest(args, success, text,
    case(
        vec![
            format!("http://127.0.0.1:{}/get", mock_server_port()),
            String::from("-v")
        ],
        true,
        predicate::str::contains("HTTP/1.1 200 OK")
    ),
    case(
        vec![
            format!("http://127.0.0.1:{}/get", mock_server_port()),
            String::from("-v")
        ],
        true,
        predicate::str::contains(format!("{}\n    {}", "content-type: application/json", "content-length: 74"))
    ),
    case(
        vec![
            format!("http://127.0.0.1:{}/get", mock_server_port())
        ],
        true,
        predicate::str::similar(test_json_response())
    ),
    case(
        vec![
            format!("http://127.0.0.1:{}/get", mock_server_port()),
            String::from("--query"),
            String::from("key=value"),
            String::from("--query"),
            String::from("key2=value2"),
            String::from("-v")
        ],
        true,
        predicate::str::is_match(r"Final URL: http://127.0.0.1:\d+/get\?key=value&key2=value2")?
    ),
    case(
        vec![
            String::from("post"),
            format!("http://127.0.0.1:{}/post", mock_server_port()),
            String::from("--basic-auth"),
            String::from("username:password"),
        ],
        true,
        predicate::str::similar("success")
    ),
    case(
        vec![
            String::from("put"),
            format!("http://127.0.0.1:{}/put", mock_server_port()),
            String::from("--bearer"),
            String::from("AaBbCcDdEeFfGg0987654321fFeEdDcCbBaA"),
        ],
        true,
        predicate::str::similar("success")
    ),
    case(
        vec![
            format!("http://127.0.0.1:{}/redirect", mock_server_port()),
            String::from("--max-redirects"),
            String::from("1")
        ],
        false,
        predicate::str::contains("too many redirects")
    ),
    case(
        vec![
            String::from("get"),
            format!("http://127.0.0.1:{}/redirect", mock_server_port()),
            String::from("--max-redirects"),
            String::from("2"),
            String::from("-v"),
        ],
        true,
        predicate::str::is_match(r"Final URL: http://127.0.0.1:\d+/redirect2")?
    ),
    case(
        vec![
            String::from("get"),
            format!("http://127.0.0.1:{}/get", mock_server_port()),
            String::from("--timeout"),
            String::from("0"),
            String::from("-v")
        ],
        false,
        predicate::str::contains("operation timed out")
    ),
    case(
        vec![
            String::from("patch"),
            format!("http://127.0.0.1:{}/patch", mock_server_port()),
            String::from("--body"),
            String::from("Some spaced body text .,?!$%&#*"),
        ],
        true,
        predicate::str::similar("success")
    ),
)]
fn test_commands<T: Predicate<str>>(args: Vec<String>, success: bool, text: T) -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::cargo_bin("qurl").unwrap().args(args).assert();
    match success {
        true => cmd.success().stdout(text),
        false => cmd.failure().stderr(text)
    };
    Ok(())
}

#[rstest(
    arg_one => [("--body",  "<body>"), ("--json", "<json>"), ("--json-file", "<json-file>"), ("--body-file", "<body-file>")],
    arg_two => [("--body",  "<body>"), ("--json", "<json>"), ("--json-file", "<json-file>"), ("--body-file", "<body-file>")],
)]
fn test_conflicts(arg_one: (&str, &str), arg_two: (&str, &str)) {
    let mut start_arg = vec!["get", "http://example.com/"];
    start_arg.extend(vec![arg_one.0, "test", arg_two.0, "test"]);
    let cmd = Command::cargo_bin("qurl")
        .unwrap()
        .args(start_arg)
        .assert()
        .failure();
    if arg_one == arg_two {
        cmd.stderr(
            predicate::str::contains(
                format!("The argument '{} {}' was provided more than once, but cannot be used multiple times", arg_one.0, arg_one.1)
            )
        );
    }
    else {
        cmd.stderr(
            predicate::str::contains(
                format!("The argument '{} {}' cannot be used with '{} {}'", arg_two.0, arg_two.1, arg_one.0, arg_one.1)
            )
        );
    }
}
