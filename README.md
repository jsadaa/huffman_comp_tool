# Huffman coding compression tool

## Description

> [!IMPORTANT]  
> **WIP**
>
> This is a work in progress and is not yet ready for use (and may never be).

(Educational and recreational) Implementation of a simple Huffman coding compression tool in Rust.

Only supports text files.

## Usage

```bash
cargo run -- [OPTION] [FILE]
```

or compile with

```bash
cargo build
```

and then

```bash
./target/debug/comp_tool [OPTION] [FILE]
```

### Details

#### Compression
The compression command creates a compressed file named `output.bin` in the same directory as the input file.

#### Decompression
The decompression command creates a decompressed file named `output.txt` in the same directory as the input file.

## Options

- `-c` compress the file
- `-d` decompress the file

## Contribution

Any remarks or suggestions are very welcome, feel free to open an issue or a pull request.