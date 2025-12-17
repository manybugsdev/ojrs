use clap::{Parser, Subcommand};
use anyhow::Result;

mod commands;
mod platforms;
mod utils;

#[derive(Parser)]
#[command(name = "ojrs")]
#[command(about = "Online Judge Tools in Rust", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download test cases from a problem URL
    Download {
        /// Problem URL
        url: String,
        
        /// Directory to save test cases (default: test/)
        #[arg(short, long, default_value = "test")]
        directory: String,
    },
    
    /// Test your code against downloaded test cases
    Test {
        /// Command to execute (e.g., "python3 solution.py")
        #[arg(short, long)]
        command: String,
        
        /// Directory containing test cases (default: test/)
        #[arg(short, long, default_value = "test")]
        directory: String,
    },
    
    /// Submit your solution to the online judge
    Submit {
        /// Problem URL
        url: String,
        
        /// Solution file path
        file: String,
    },
    
    /// Login to an online judge platform
    Login {
        /// Platform URL
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Download { url, directory } => {
            commands::download::execute(&url, &directory).await?;
        }
        Commands::Test { command, directory } => {
            commands::test::execute(&command, &directory)?;
        }
        Commands::Submit { url, file } => {
            commands::submit::execute(&url, &file).await?;
        }
        Commands::Login { url } => {
            commands::login::execute(&url).await?;
        }
    }
    
    Ok(())
}
