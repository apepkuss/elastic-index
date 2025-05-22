use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to read files from
    #[arg(short, long)]
    path: String,

    /// Output file path for bulk data
    #[arg(short, long, default_value = "bulk.jsonl")]
    output: String,

    /// Elasticsearch index name
    #[arg(short, long, default_value = "documents")]
    index: String,
}

fn main() {
    let args = Args::parse();

    if let Ok(docs) = read_files_from_directory(&args.path) {
        // Create output file
        if let Ok(mut file) = std::fs::File::create(&args.output) {
            for doc in docs {
                // Create index action line
                let index_action = serde_json::json!({
                    "index": {
                        "_index": args.index,
                        "_id": doc.title
                    }
                });

                // Write index action line
                if let Ok(index_line) = serde_json::to_string(&index_action) {
                    writeln!(file, "{}", index_line).unwrap_or_default();
                }

                // Write document data line
                if let Ok(doc_line) = serde_json::to_string(&doc) {
                    writeln!(file, "{}", doc_line).unwrap_or_default();
                }
            }
            println!("Successfully wrote bulk data to {}", args.output);
        } else {
            eprintln!("Error: Failed to create output file: {}", args.output);
        }
    } else {
        eprintln!("Error: Failed to read files from directory: {}", args.path);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MyDoc {
    title: String,
    content: String,
}

fn read_files_from_directory(dir_path: &str) -> std::io::Result<Vec<MyDoc>> {
    let mut docs = Vec::new();
    let dir = Path::new(dir_path);

    // Collect all file paths
    let mut file_paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    // Sort files by name
    file_paths.sort_by(|a, b| {
        a.file_name()
            .and_then(|na| na.to_str())
            .unwrap_or("")
            .cmp(&b.file_name().and_then(|nb| nb.to_str()).unwrap_or(""))
    });

    // Process files in sorted order
    for path in file_paths {
        // Get file name without extension
        let file_name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Read file content
        let content = fs::read_to_string(&path)?;

        // Create MyDoc instance
        let doc = MyDoc {
            title: file_name,
            content,
        };

        docs.push(doc);
    }

    Ok(docs)
}
