use chrono::Local;
use serde::Deserialize;
use std::{env, fs, path::Path, process::Command};

const TEMPLATE_CONTENT: &str = include_str!("../ch-letter-template.typ");

#[derive(Deserialize, Default)]
struct Defaults {
    sender: Option<SenderProfiles>,
    location: Option<String>,
    lang: Option<String>,
}

#[derive(Deserialize, Default)]
struct SenderProfiles {
    private: Option<Sender>,
    work: Option<Sender>,
}

#[derive(Deserialize, Default, Clone)]
struct Sender {
    name: Option<String>,
    address: Option<String>,
    extra: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Profile {
    Private,
    Work,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                eprintln!("Usage: briefli new [-w|--work] <subject>");
                std::process::exit(1);
            }

            let (profile, subject) = parse_new_args(&args[2..]);
            new_letter(&subject, profile);
        }
        "build" => build_all(),
        "list" => list_letters(),
        "init" => init_directory(),
        "help" | "--help" | "-h" => print_help(),
        "--version" | "-V" => println!("briefli {}", env!("CARGO_PKG_VERSION")),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn parse_new_args(args: &[String]) -> (Profile, String) {
    let mut profile = Profile::Private;
    let mut subject_parts = vec![];

    for arg in args {
        match arg.as_str() {
            "-w" | "--work" => profile = Profile::Work,
            "-p" | "--private" => profile = Profile::Private,
            _ => subject_parts.push(arg.clone()),
        }
    }

    (profile, subject_parts.join(" "))
}

fn print_help() {
    println!(
        r#"briefli - Swiss letter management CLI

USAGE:
    briefli <command> [args]

COMMANDS:
    new [flags] <subject>   Create a new letter (YYYY-MM-DD Subject.typ)
    build                   Compile all .typ files to PDF
    list                    List all letters
    init                    Initialize a new letters directory
    help                    Show this help

FLAGS (for 'new'):
    -p, --private    Use private address (default)
    -w, --work       Use work address

EXAMPLES:
    briefli init
    briefli new "Kündigung Mietvertrag"
    briefli new -w "Projektanfrage"
    briefli build
"#
    );
}

fn load_defaults() -> Defaults {
    fs::read_to_string("defaults.toml")
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn get_sender(defaults: &Defaults, profile: Profile) -> Option<Sender> {
    defaults.sender.as_ref().and_then(|profiles| match profile {
        Profile::Private => profiles.private.clone(),
        Profile::Work => profiles.work.clone(),
    })
}

fn init_directory() {
    // Create template file
    let template_path = Path::new("ch-letter-template.typ");
    if template_path.exists() {
        println!("ch-letter-template.typ already exists, skipping");
    } else {
        fs::write(template_path, TEMPLATE_CONTENT).expect("Failed to write template");
        println!("Created: ch-letter-template.typ");
    }

    // Create defaults file
    let defaults_path = Path::new("defaults.toml");
    if defaults_path.exists() {
        println!("defaults.toml already exists, skipping");
    } else {
        let defaults_content = r#"# Default values for new letters
# Edit these to match your details

# Default location for the date line
location = "Zürich"

# Default language (de, fr, it, en)
lang = "de"

[sender.private]
name = "Your Name"
address = "Street 123, 8000 Zürich"
# extra = "+41 79 123 45 67"  # Optional: phone, email, etc.
# location = "Zürich"  # Optional: override global location

[sender.work]
name = "Your Name"
address = "Company AG, Street 456, 8001 Zürich"
# extra = "your.email@company.ch"
# location = "Zürich"  # Optional: override global location
"#;
        fs::write(defaults_path, defaults_content).expect("Failed to write defaults");
        println!("Created: defaults.toml");
    }

    println!(
        "\n✓ Initialized! Edit defaults.toml with your details, then run: briefli new \"Subject\""
    );
}

fn new_letter(name: &str, profile: Profile) {
    // Check if template exists
    if !Path::new("ch-letter-template.typ").exists() {
        eprintln!("Error: ch-letter-template.typ not found");
        eprintln!("Run 'briefli init' to create it");
        std::process::exit(1);
    }

    let defaults = load_defaults();
    let date = Local::now().format("%Y-%m-%d").to_string();
    let display_date = Local::now().format("%d.%m.%Y").to_string();
    let filename = format!("{} {}.typ", date, name);

    if Path::new(&filename).exists() {
        eprintln!("Error: {} already exists", filename);
        std::process::exit(1);
    }

    let lang = defaults.lang.clone().unwrap_or_else(|| "de".to_string());
    let sender = get_sender(&defaults, profile);

    // Use profile-specific location if set, otherwise fall back to global location
    let location = sender
        .as_ref()
        .and_then(|s| s.location.clone())
        .or_else(|| defaults.location.clone())
        .unwrap_or_else(|| "Zürich".to_string());

    // Build sender block
    let sender_block = if let Some(s) = &sender {
        let mut parts = vec![];
        if let Some(name) = &s.name {
            parts.push(format!("    name: \"{}\",", name));
        }
        if let Some(addr) = &s.address {
            parts.push(format!("    address: \"{}\",", addr));
        }
        if let Some(extra) = &s.extra {
            parts.push(format!("    extra: \"{}\",", extra));
        }
        if parts.is_empty() {
            String::new()
        } else {
            format!("  sender: (\n{}\n  ),\n", parts.join("\n"))
        }
    } else {
        String::new()
    };

    // Get sender name for signature
    let sender_name = sender
        .as_ref()
        .and_then(|s| s.name.clone())
        .unwrap_or_default();

    let content = format!(
        r#"#import "ch-letter-template.typ": ch-letter

#set text(lang: "{lang}")

#show: ch-letter.with(
{sender_block}
  recipient: "",

  location: "{location}",
  date: "{display_date}",
  subject: "{subject}",
)

Sehr geehrte Damen und Herren



Freundliche Grüsse

#v(1.5cm)
{sender_name}
"#,
        lang = lang,
        sender_block = sender_block,
        location = location,
        display_date = display_date,
        subject = name,
        sender_name = sender_name,
    );

    fs::write(&filename, content).expect("Failed to write file");
    let profile_name = match profile {
        Profile::Private => "private",
        Profile::Work => "work",
    };
    println!("Created: {} ({})", filename, profile_name);
}

fn build_all() {
    let entries = match fs::read_dir(".") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };

    let mut count = 0;
    let mut skipped = 0;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "typ") {
            let name = path.file_name().unwrap().to_string_lossy();
            // Skip template files
            if name.ends_with("-template.typ") {
                continue;
            }
            // Skip if PDF already exists
            let pdf_path = path.with_extension("pdf");
            if pdf_path.exists() {
                skipped += 1;
                continue;
            }
            compile_typ(&path.to_string_lossy());
            count += 1;
        }
    }

    if count == 0 && skipped == 0 {
        println!("No .typ files to compile");
    } else if count == 0 {
        println!("Nothing to build ({} already have PDFs)", skipped);
    } else {
        println!("\nCompiled {} letter(s), {} skipped", count, skipped);
    }
}

fn compile_typ(path: &str) {
    print!("Compiling {}... ", path);

    let status = Command::new("typst").args(["compile", path]).status();

    match status {
        Ok(s) if s.success() => println!("✓"),
        Ok(s) => {
            println!("✗");
            if let Some(code) = s.code() {
                eprintln!("  typst exited with code {}", code);
            }
        }
        Err(e) => {
            println!("✗");
            eprintln!("  Failed to run typst: {}", e);
            eprintln!("  Make sure typst is installed and in your PATH");
        }
    }
}

fn list_letters() {
    let mut letters: Vec<String> = fs::read_dir(".")
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if path.extension().map_or(false, |e| e == "typ") {
                let name = path.file_name()?.to_string_lossy().to_string();
                if !name.ends_with("-template.typ") {
                    return Some(name);
                }
            }
            None
        })
        .collect();

    letters.sort();

    if letters.is_empty() {
        println!("No letters found");
        return;
    }

    println!("Letters:\n");
    for letter in letters {
        let pdf = letter.replace(".typ", ".pdf");
        let has_pdf = Path::new(&pdf).exists();
        let status = if has_pdf { "✓" } else { "○" };
        println!("  {} {}", status, letter.replace(".typ", ""));
    }
    println!("\n  ✓ = PDF exists  ○ = needs build");
}
