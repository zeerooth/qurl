use qurl::RequestParser;
use qurl::debug::PrettyPrint;
use qurl::cli::app_matches;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(match run_app().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red(), err);
            1
        }
    });

}

async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let matches = app_matches();
    let verbose = matches.is_present("verbose");
    let built_request = match RequestParser::new(matches) {
        Ok(req_parser) => req_parser,
        Err(err) => return Err(err.inner())
    };
    if verbose { println!("{}\n{}", "Making a request:".green().bold(), built_request.prettify()?); }
    let response = built_request.send().await?;
    if verbose { println!("{}\n{}", "Received response:".green().bold(), response.prettify().unwrap()); }
    print!("{}", response.text().await?); // TODO: make sure to handle timeout of body transfer!
    if verbose { println!("{}", "\nProgram finished successfully".green().bold()); }
    Ok(())
}