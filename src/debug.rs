use reqwest::{Response};
use colored::*;
use super::RequestParser;
use std::fmt::Write;

pub trait PrettyPrint {
    fn prettify(&self) -> Result<String, Box<dyn std::error::Error>>;
}

impl PrettyPrint for RequestParser {
    fn prettify(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut f = String::new();
        writeln!(f, "{}: {}", "Method".cyan(), self.request.method())?;
        writeln!(f, "{}: {}", "URL".cyan(), self.request.url())?;
        if let Some(timeout) = self.request.timeout() {
            writeln!(f, "{}: {} ms", "Timeout".cyan(), timeout.as_millis())?;
        }
        writeln!(f, "{}:", "Headers".cyan())?;
        for header in self.request.headers() {
            writeln!(f, "{}{}: {}", " ".repeat(4), header.0.as_str().cyan(), header.1.to_str().unwrap())?;
        }
        Ok(f)
    }
}

impl PrettyPrint for Response {
    fn prettify(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut f = String::new();
        let status_code = self.status().as_u16();
        let status_code_str = match status_code {
            200 ..= 299 => status_code.to_string().green(),
            300 ..= 399 => status_code.to_string().yellow(),
            400 ..= 599 => status_code.to_string().bright_red(),
            _ => status_code.to_string().blue()
        };
        writeln!(
            f,
            "{version} {code} {reason}",
            version=format!("{:?}", self.version()).bright_magenta(),
            code=status_code_str,
            reason=self.status().canonical_reason().unwrap_or("")
        )?;
        writeln!(f, "{}: {}", "Final URL".cyan(), self.url())?;
        if let Some(remote_addr) = self.remote_addr() {
            writeln!(f, "{}: {}", "Remote IP Address".cyan(), remote_addr)?;
        };
        writeln!(f, "{}:", "Headers".cyan())?;
        for header in self.headers() {
            writeln!(f, "{}{}: {}", " ".repeat(4), header.0.as_str().cyan(), header.1.to_str().unwrap())?;
        }
        Ok(f)
    }
}