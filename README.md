# Create Elasticsearch Index

> [!TIP]
> This tool is used to create a JSONL file from a directory of files. The JSONL file can be used to create an Elasticsearch index.

## Usage

```bash
cargo run -- --path ./<path-to-your-files> --output ./<your-output-file>.jsonl --index <your-index-name>
```

## Parameters

- `path`: Path to the input files
- `output`: Path to the output file. File extension should be `.jsonl`
- `index`: Name of the Elasticsearch index
