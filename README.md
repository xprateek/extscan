# extscan

**extscan** is a command-line tool for analyzing file extensions in directories with support for colored output, Nerd Font icons, counts next to extensions, and sorting options. It aids developers and users in quickly identifying the types and prevalence of files within a directory.

***

## Features

- Scan directories to count and list file extensions
- Support for colored terminal output
- Optional Nerd Font icons for each extension
- Display counts of files per extension
- Sort extensions alphabetically or by count
- Include or exclude hidden files and dotfiles
- List dotfiles separately for easy visibility
- Cross-platform support (Linux and macOS, many architectures)

***

## Installation

Build from source using Cargo:

```bash
cargo build --release
```

Or download prebuilt binaries from the [GitHub Releases](https://github.com/xprateek/extscan/releases).

***

## Usage

```bash
extscan [OPTIONS] [path]
```

- `[path]` - Directory to scan (default: current directory `"."`)

### Options

| Flag | Description                                  |
|-------|---------------------------------------------|
| `-c`  | Enable colored output                        |
| `-n`  | Enable Nerd Font icons                       |
| `-x`  | Show counts next to extensions               |
| `-1`  | Sort files alphabetically, dotfiles first   |
| `-h`  | Include hidden files and dotfiles            |
| `-l`  | List dotfiles separately, then extensions    |
| `--help` | Print help                              |
| `-V, --version` | Print version                          |

![Terminal macOS screenshot](https://raw.githubusercontent.com/xprateek/extscan/refs/heads/main/screenshot/terminal_macos.png)

![Terminal macOS Screen Recording](https://raw.githubusercontent.com/xprateek/extscan/refs/heads/main/screenshot/Screen-Recording_macOS-terminal.mov)

***

## Examples

Scan current directory and show extensions with counts and icons:

```bash
extscan -cnx1
```

Scan `/path/to/dir`, include hidden files, sorted alphabetically:

```bash
extscan -h1 /path/to/dir
```

List dotfiles separately and include hidden files:

```bash
extscan -hl
```

***

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

***

## Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check [issues page](https://github.com/xprateek/extscan/issues) or submit pull requests.

***

## Author and Source

Created by Prateek Maru  
Repository: [https://github.com/xprateek/extscan](https://github.com/xprateek/extscan)

***

## Acknowledgments

- Powered by Rust and Clap  
- Nerd Font icons for enhanced visuals  
- Inspired by Unix-style CLI tools and modern cross-architecture releases

***

