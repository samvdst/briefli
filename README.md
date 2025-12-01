# briefli 🇨🇭

[![Crates.io](https://img.shields.io/crates/v/briefli.svg)](https://crates.io/crates/briefli)
[![License](https://img.shields.io/crates/l/briefli.svg)](https://github.com/samvdst/briefli#license)

Swiss letter management CLI using [Typst](https://typst.app/). Create properly formatted letters that fit Swiss envelope windows (C5/C6 with left-side address window).

## Features

- 📬 Swiss standard letter format (SN 010130)
- 📍 Address positioning for left-side envelope windows
- 👤 Multiple sender profiles (private/work)
- 🔤 Typst-based templates for beautiful typography
- 📦 Simple CLI for letter management

## Installation

### From crates.io

```bash
cargo install briefli
```

### From source

```bash
git clone https://github.com/samvdst/briefli
cd briefli
cargo install --path .
```

### Prerequisites

- [Typst](https://github.com/typst/typst#installation) must be installed and in your PATH

## Quick Start

1. **Initialize your letters directory:**

```bash
mkdir my-letters && cd my-letters
briefli init
```

2. **Configure your sender info** in `defaults.toml`:

```toml
[sender.private]
name = "Your Name"
address = "Street 123, 8000 Zürich"

[sender.work]
name = "Your Name"
address = "Company AG, Street 456, 8001 Zürich"

place = "Zürich"
lang = "de"
```

3. **Create a new letter:**

```bash
briefli new "Kündigung Mietvertrag"
# Creates: 2024-01-15 Kündigung Mietvertrag.typ
```

4. **Edit the `.typ` file**, then compile:

```bash
briefli build
```

## Usage

```bash
briefli new <subject>           # Create letter with private address (default)
briefli new -w <subject>        # Create letter with work address
briefli new --work <subject>    # Same as above

briefli build                   # Compile all .typ files to PDF
briefli list                    # List all letters and their status
briefli init                    # Initialize a new letters directory
briefli help                    # Show help
```

## Swiss Letter Format

The template follows Swiss letter standards:

| Element           | Position      |
| ----------------- | ------------- |
| Left margin       | 22mm          |
| Recipient address | 60mm from top |
| Address window    | 85.5mm × 45mm |

This ensures your letters fit perfectly in standard Swiss C5/C6 envelopes with the left-side address window.

## File Structure

```
my-letters/
├── defaults.toml              # Your sender configuration
├── ch-letter-template.typ     # Swiss letter template
├── 2024-01-15 Letter.typ      # Letter source
└── 2024-01-15 Letter.pdf      # Compiled PDF
```

## Template Customization

The `ch-letter-template.typ` can be customized. Available parameters:

```typst
#show: ch-letter.with(
  sender: (
    name: "Your Name",
    address: "Street, ZIP City",
    extra: "phone/email",       // optional
  ),
  recipient: "Name, Company, Street, ZIP City",
  location: "Zürich",
  date: "15.01.2024",
  subject: "Letter Subject",
  footer: [Optional footer],    // optional
  font: "Arial",                // optional, default: Arial
)
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
