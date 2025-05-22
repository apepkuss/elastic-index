# Create Elasticsearch Index

> [!TIP]
> This tool is used to create an Elasticsearch index from a directory of files.

## Usage

```bash
cargo run -- --path ./<path-to-your-files> --output ./<your-output-file>.jsonl --index <your-index-name>
```

## Parameters

- `path`: Path to the input files
- `output`: Path to the output file. File extension should be `.jsonl`
- `index`: Name of the Elasticsearch index
