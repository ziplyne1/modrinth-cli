use clap::Parser;

#[derive(Parser)]
struct Cli {
    mod_name: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let url: String = format!(
        "https://api.modrinth.com/v2/project/{}/version",
        args.mod_name
    );
    let response: reqwest::Response = reqwest::get(&url).await.unwrap();
    let body: String = response.text().await.unwrap();

    println!("Fetching: {url}");
    println!("{body}");
}
