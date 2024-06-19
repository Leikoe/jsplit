# JSON Array File Splitter

## Overview

This Rust CLI tool is designed to split large JSON array files into smaller shards for easier processing and handling. Each shard contains a specified number of entries from the original JSON file.

## Features

- Splits large JSON array files into smaller, manageable shards.
- Configurable number of entries per shard.
- Outputs shards into a specified directory.

## Installation

To install this CLI tool, you need to have Rust and Cargo installed on your machine. If you don't have them installed, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone [<repository_url>](https://github.com/Leikoe/jsplit.git)
cd jsplit
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

Run the tool from the command line with the following options:

```sh
jsplit [OPTIONS] --input-path <INPUT_PATH>
```

### Options

- `-i, --input-path <INPUT_PATH>`: Path of the JSON file to split (required).
- `-s, --shards-size <SHARDS_SIZE>`: Number of entries per shard (default: 1024).
- `-o, --output-path <OUTPUT_PATH>`: Path of the output folder (default: `./shards/`).

### Example

```sh
./jsplit -i large_file.json -s 1000 --o ./output_shards/
```

This command will split `large_file.json` into shards of 1000 entries each and save them in the `./output_shards/` directory.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
