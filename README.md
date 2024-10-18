# text-figlet

`text-figlet` is a simple Rust command-line application that converts text into FIGlet-style ASCII art using the `figleter` crate.

## Features

- Converts a single line of text into ASCII art.
- Uses the standard FIGlet font to render the text.

## Requirements

- Rust (edition 2021 or newer)
- The `figleter` crate version 0.2.2

## Installation

To build and run the application, make sure you have Rust installed. Then, clone the repository and run:

```bash
git clone https://github.com/V-Karch/text-figlet
cd text-figlet
cargo build --release
```

## Usage
Run the program using the following command:
```bash
./text-figlet <text-argument>
```

## Example

```bash
./text-figlet Hello!
```
## Result
```bash
  _   _      _ _       _ 
 | | | | ___| | | ___ | |
 | |_| |/ _ \ | |/ _ \| |
 |  _  |  __/ | | (_) |_|
 |_| |_|\___|_|_|\___/(_)
                         
```

## Dependencies

This project relies on the following crate:
- [`figleter`](https://crates.io/crates/figleter) = "0.2.2"

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
