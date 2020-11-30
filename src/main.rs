use qurl::RequestParser;
use qurl::debug::PrettyPrint;
use qurl::cli::app_matches;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = app_matches();
    let verbose = matches.is_present("verbose");
    let built_request = match RequestParser::new(matches) {
        Ok(req_parser) => req_parser,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red(), err.inner());
            std::process::exit(1)
        }
    };
    if verbose { println!("{}\n{}", "Making a request:".green().bold(), built_request.prettify()?); }
    let response = built_request.send().await?;
    if verbose { println!("{}\n{}", "Received response:".green().bold(), response.prettify().unwrap()); }
    print!("{}", response.text().await?);
    if verbose { println!("{}", "\nProgram finished successfully".green().bold()); }
    Ok(())
}