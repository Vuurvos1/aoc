use std::env;
use std::fs;
use std::path::Path;

use chrono::Datelike;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin fetch_input <day> [year]");
        eprintln!("Example: cargo run --bin fetch_input 1");
        eprintln!("Example: cargo run --bin fetch_input 1 2024");
        std::process::exit(1);
    }

    let day = args[1].parse::<u32>().map_err(|_| "Invalid day number")?;

    let year = if args.len() > 2 {
        args[2].parse::<u32>().map_err(|_| "Invalid year")?
    } else {
        chrono::Utc::now().year() as u32 // Default to current year
    };

    // Validate day range
    if day < 1 || day > 25 {
        eprintln!("Error: Day must be between 1 and 25");
        std::process::exit(1);
    }

    // Get session token from environment
    let session_token = env::var("AOC_SESSION")
        .or_else(|_| env::var("SESSION"))
        .map_err(|_| "Session token not found. Set AOC_SESSION or SESSION environment variable")?;

    fetch_input(day, year, &session_token).await?;

    Ok(())
}

async fn fetch_input(
    day: u32,
    year: u32,
    session_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("inputs/d{:02}.txt", day);
    let filepath = Path::new(&filename);

    // Check if file already exists
    if filepath.exists() {
        println!("Input file {} already exists, skipping", filename);
        return Ok(());
    }

    // Create inputs directory if it doesn't exist
    if let Some(parent) = filepath.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Fetching input for day {} of year {}...", day, year);

    let client = reqwest::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .header("User-Agent", "github.com/adventOfCode-fetcher")
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Error fetching input: HTTP {}", response.status());
        eprintln!("Make sure your session token is valid and the day is available");
        std::process::exit(1);
    }

    let content = response.text().await?;

    // Write to file
    fs::write(filepath, content)?;

    println!("Successfully saved input to {}", filename);

    Ok(())
}
