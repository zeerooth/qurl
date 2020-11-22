use qurl::RequestParser;
use qurl::cli::app_matches;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create application like normal
    let matches = app_matches();
    let verbose = matches.is_present("verbose");
    let built_request = match RequestParser::new(matches) {
        Ok(req_parser) => req_parser,
        Err(err) => return Err(err.inner())
    };
    if verbose { println!("{}\n{:#?}", "[DEBUG] Making a request:".green(), built_request); }
    let response = built_request.send().await?;
    if verbose { println!("{}\n{:#?}", "[DEBUG] Received response:".green(), response); }
    println!("{}", response.text().await?);
    if verbose { println!("{}", "[DEBUG] Program finished successfully".green()); }
    Ok(())
}