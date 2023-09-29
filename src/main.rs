use std::collections::HashMap;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Print all .env variables
    List,

    /// Set environment
    Set { key: String, value: String },

    // don't use flags
    /// Get environment
    Get { key: String },
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,

    /// Set stage
    #[clap(short, long, global = true)]
    stage: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut filename = ".env".to_string();

    if let Some(stage) = args.stage {
        filename = format!(".env.{}", stage);
    }

    let has_dotenv = tokio::fs::try_exists(&filename).await?;

    if !has_dotenv {
        println!("No {filename} file found");
        return Ok(());
    }

    match args.cmd {
        SubCommand::List => {
            let values = get_env(&filename).await?;

            for (key, value) in values {
                println!("{}={}", key, value);
            }
        }
        SubCommand::Get { key } => {
            let values = get_env(&filename).await?;

            if let Some(value) = values.get(&key) {
                println!("{}", value);
            } else {
                println!("No value found for key: {}", key);
            }
        }
        SubCommand::Set { key, value } => {
            let mut values = get_env(&filename).await?;

            values.insert(key.clone(), value.clone());

            let mut file_content = String::new();

            for (key, value) in values {
                file_content.push_str(&format!("{}={}\n", key, value));
            }

            tokio::fs::write(".env", file_content).await?;

            println!("{}={}", key, value);
        }
    }

    Ok(())
}

async fn get_env(filename: &str) -> anyhow::Result<HashMap<String, String>> {
    // find .env file in the cwd
    let file_content = tokio::fs::read_to_string(filename).await?;
    let mut hashmap: HashMap<String, String> = HashMap::new();

    file_content.split('\n').for_each(|row| {
        // skip comments
        if row.starts_with('#') {
            return;
        };

        if let Some((a, b)) = row.split_once('=') {
            hashmap.insert(a.to_string(), b.to_string());
        }
    });

    Ok(hashmap)
}
