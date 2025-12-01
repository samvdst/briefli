# briefli 🇨🇭

[![Crates.io](https://img.shields.io/crates/v/briefli.svg)](https://crates.io/crates/briefli)
[![License](https://img.shields.io/crates/l/briefli.svg)](https://github.com/samvdst/briefli#license)

Swiss letter management CLI using [Typst](https://typst.app/). Create properly formatted letters for Swiss C5 envelopes (A4 folded once) with left or right address windows.

## Features

- 📬 Swiss standard letter format (SN 010130)
- 📍 Address positioning for left or right-side envelope windows
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
location = "Zürich"
lang = "de"
address-position = "left"  # or "right"

[sender.private]
name = "Your Name"
address = "Street 123, 8000 Zürich"

[sender.work]
name = "Your Name"
address = "Company AG, Street 456, 8001 Zürich"
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

For C5 envelopes (A4 folded once), the template uses these measurements:

| Element              | Left Window   | Right Window  |
| -------------------- | ------------- | ------------- |
| Address from left    | 22mm          | 118mm         |
| Address from top     | 60mm          | 60mm          |
| Address area         | 85.5mm × 45mm | 85.5mm × 45mm |
| Sender line from top | 45mm          | 45mm          |

Use `address-position = "left"` (default) or `"right"` depending on your envelope's window position.

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
    extra: "phone/email",          // optional
  ),
  recipient: "Name, Company, Street, ZIP City",
  location: "Zürich",
  date: "15.01.2024",
  subject: "Letter Subject",
  footer: [Optional footer],       // optional
  font: "Arial",                   // optional, default: Arial
  address-position: "left",        // optional, "left" or "right"
)
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
