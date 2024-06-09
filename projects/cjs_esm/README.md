# cesm

`cesm` is a command-line tool designed to process TypeScript files by adding `.js` to the import and export paths. This can be useful when migrate a TypeScript projects from Commonjs to ESM.

## Features

- **Add `.js` to import/export paths**: Automatically modifies TypeScript files to include `.js` extensions in import and export statements.
- **Glob pattern matching**: Specify patterns to match files using glob syntax.
- **User-friendly CLI**: Intuitive and easy-to-use command-line interface powered by `clap`.

## Installation

- To install `cesm`, you need to have Rust and Cargo installed. Then you can build the project from the source:

```sh
git clone https://github.com/tduyng/rthings.git
cd rthings/projects/cjs_esm
cargo install --path .
```
- You can also install directly from the GitHub repository using the following command:

```sh
git install --git https://github.com/tduyng/rthings/tree/main/projects/cjs_esm
```

## Usage

### Commands

#### `addjs`

This command processes TypeScript files to add `.js` to import/export paths.

```sh
cesm addjs <PATTERN>
```

- **PATTERN**: The glob pattern to match TypeScript files.

### Examples

Process all TypeScript files in the `src` directory and its subdirectories:

```sh
cesm addjs "src/**/*.ts"
```

Process a specific file:

```sh
cesm addjs "src/main.ts"
```

### Notes

- If no files matching the pattern were modified, a message indicating this will be printed.
- If files are modified, the count of processed files will be printed.

## Development

### Dependencies

- **[clap](https://github.com/clap-rs/clap)**: For command-line argument parsing.
- **[anyhow](https://github.com/dtolnay/anyhow)**: For error handling.
- **[glob](https://github.com/rust-lang-nursery/glob)**: For file pattern matching.
- **[regex](https://github.com/rust-lang/regex)**: For regular expression processing.

### Running Locally

To run the project locally for development purposes:

```sh
cargo run -- addjs "src/**/*.ts"
```