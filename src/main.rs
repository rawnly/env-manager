mod cli;

use clap::Parser;
use cli::{Cli, Format, ListArgs, SubCommand};
use std::collections::HashMap;

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
        SubCommand::ListFiles => {
            let mut entries = tokio::fs::read_dir(".").await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();

                if path.is_file()
                    && path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .starts_with(".env")
                {
                    println!("{}", path.file_name().unwrap().to_string_lossy());
                }
            }
        }

        SubCommand::List(ListArgs { format }) => {
            let values = get_env(&filename).await?;

            match format {
                Some(Format::Json) => {
                    let json_string = serde_json::to_string_pretty(&values)?;
                    println!("{}", json_string);
                }
                Some(Format::Yaml) => {
                    let yaml_string = serde_yaml::to_string(&values)?;
                    println!("{}", yaml_string);
                }
                None => {
                    for (key, value) in values {
                        println!("{}={}", key, value);
                    }
                }
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

            tokio::fs::write(&filename, file_content).await?;

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
