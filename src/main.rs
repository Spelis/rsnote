use chrono::Local;
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use std::process::Command as ProcCommand;
use std::process::exit;
use std::{env, fs};

fn write_notes(file: &str, data: Notes) {
    fs::write(&file, serde_json::to_string(&data).unwrap()).expect("❌ Failed to write.");
}

fn read_notes(file: &str) -> Notes {
    let content = fs::read_to_string(file).expect("❌ Failed to read.");
    let json: Notes = serde_json::from_str(&content).expect("❌ File may be corrupted.");
    return json;
}

fn trim_multiline(input: &str) -> String {
    input
        .lines()
        .skip(1)
        .skip_while(|l| l.trim().is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip_while(|l| l.trim().is_empty())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Unable to read user input");
    input.trim().to_string()
}

fn confirm(prompt: &str) -> bool {
    input(prompt).eq_ignore_ascii_case("y")
}

fn open_editor(path: &std::path::Path) -> std::io::Result<()> {
    let editors = ["nvim", "vim", "nano", "vi", "edit"];

    for editor in editors {
        if which::which(editor).is_ok() {
            ProcCommand::new(editor).arg(path).status()?;
            return Ok(());
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "❌ No suitable editor found in $PATH.",
    ))
}

fn cmd_init(file: String, _args: Vec<String>) {
    fs::File::create(&file).expect("❌ Failed to create file.\n");
    let data = Notes(vec![]);
    write_notes(&file, data);
    println!("✅ Notefile initialized: {}", &file);
}
fn cmd_list(file: String, _args: Vec<String>) {
    let notes: Notes = read_notes(&file);
    for note in notes.0.iter() {
        println!("Note {}: {}", &note.id, &note.title)
    }
}
fn cmd_new(file: String, args: Vec<String>) {
    let mut notes: Notes = read_notes(&file);
    let id = args
        .get(0)
        .expect("❌ ID Required, can be number or string.");

    if notes.0.iter().any(|n| n.id == *id) {
        eprintln!("❌ Note with id {} already exists.", id);
        return;
    }

    let mut tmp = tempfile::NamedTempFile::new().expect("❌ failed to create temp file.");
    writeln!(tmp, "<title>\n\n<body>").expect("❌ Failed to write");
    open_editor(tmp.path()).expect("❌ Failed to open");
    let content = fs::read_to_string(tmp).expect("❌ Failed to read");
    let title = content.lines().next().unwrap_or("Untitled");
    let body = trim_multiline(&content);
    if body == "<body>" && title == "<title>" {
        println!("❌ Did not create empty note.");
        return;
    }
    let now = Local::now().timestamp();
    notes.0.push(Note {
        title: title.into(),
        body: body,
        id: id.to_string(),
        created_at: now,
        edited_at: now,
        tags: vec![],
    });
    write_notes(&file, notes);
    println!("✅ Successfully created {}", id);
}
fn cmd_edit(file: String, args: Vec<String>) {
    let mut notes: Notes = read_notes(&file);
    let id = args
        .get(0)
        .expect("❌ ID Required, can be number or string.");

    let Some(note) = notes.0.iter_mut().find(|n| &n.id == id) else {
        eprintln!("❌ Note with ID '{}' not found.", id);
        return;
    };

    let mut tmp = tempfile::NamedTempFile::new().expect("❌ Failed to create temp file.");
    writeln!(tmp, "{}\n\n{}", note.title, note.body).expect("❌ Failed to write");
    open_editor(tmp.path()).expect("❌ Failed to open");
    let content = fs::read_to_string(tmp).expect("❌ Failed to read");
    let title = content.lines().next().unwrap_or("Untitled");
    let body = trim_multiline(&content);
    note.title = title.to_string();
    note.body = body;
    note.edited_at = Local::now().timestamp();
    if confirm("Are you sure you want to edit? (y/N): ") {
        write_notes(&file, notes);
    }
    println!("✅ Successfully edited {}", id);
}

fn cmd_delete(file: String, args: Vec<String>) {
    let mut notes: Notes = read_notes(&file);
    let id = args
        .get(0)
        .expect("❌ ID Required, can be number or string.");
    if confirm("Are you sure? (y/N): ") {
        let noteidx = notes
            .0
            .iter()
            .position(|n| &n.id == id)
            .expect("❌ Note not found.");
        notes.0.remove(noteidx);
        write_notes(&file, notes);
        println!("✅ Successfully deleted {}", id);
    } else {
        println!("✅ Canceled.");
    }
}

fn cmd_search(file: String, args: Vec<String>) {
    let notes: Notes = read_notes(&file);
    let query = args.get(0).map(|s| s.as_str()).unwrap_or("").to_lowercase();

    if query.is_empty() {
        println!("❌ No query provided.");
        return;
    }

    fn highlight(text: &str, query: &str) -> String {
        let mut result = String::new();
        let mut remaining = text.to_string();
        let query_lc = query.to_lowercase();

        while let Some(pos) = remaining.to_lowercase().find(&query_lc) {
            let (before, rest) = remaining.split_at(pos);
            let (matched, after) = rest.split_at(query.len());
            result.push_str(before);
            result.push_str("\x1b[31m"); // red
            result.push_str(matched);
            result.push_str("\x1b[0m"); // reset
            remaining = after.to_string();
        }
        result.push_str(&remaining);
        result
    }

    let result: Vec<(usize, &Note)> = notes
        .0
        .iter()
        .enumerate()
        .filter(|(_, note)| {
            note.title.to_lowercase().contains(&query)
                || note.body.to_lowercase().contains(&query)
                || note.id.to_lowercase().contains(&query)
        })
        .collect();

    println!(
        "🔍 Search for \"{}\" returned {} results:",
        query,
        result.len()
    );

    for (_, note) in result {
        println!(
            "{}: {}",
            highlight(&note.id, &query),
            highlight(&note.title, &query)
        );
        for line in note.body.lines() {
            println!("    {}", highlight(line, &query));
        }
    }
}

fn cmd_info(file: String, args: Vec<String>) {
    let mut notes: Notes = read_notes(&file);
    let id = args
        .get(0)
        .expect("❌ ID Required, can be number or string.");

    let Some(note) = notes.0.iter_mut().find(|n| &n.id == id) else {
        eprintln!("❌ Note with ID '{}' not found.", id);
        return;
    };

    println!("ℹ️ Info for note {}:", note.id);
    println!("    Title: {}", note.title);
    println!("    Body:");
    for line in note.body.lines() {
        println!("        {}", line);
    }
    println!(
        "    Last Modified: {}",
        Local
            .timestamp_opt(note.edited_at as i64, 0)
            .single()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "    Created at:    {}",
        Local
            .timestamp_opt(note.created_at as i64, 0)
            .single()
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
    );
    println!("    Tags: {}", note.tags.join(", "))
}

fn cmd_about(_file: String, _args: Vec<String>) {
    println!("RSNote - Rust note taking app");
    println!("Very simple Command line interface to manage notes.");
    println!("\nMade by Spelis");
}

macro_rules! commands {
    ( $( $name:expr => $func:ident $( ( $usage:expr ) )? ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert(
                $name.to_string(),
                Command {
                    func: $func,
                    usage: commands!(@usage $($usage)?),
                },
            );
        )*
        map
    }};

    (@usage $usage:expr) => { $usage.to_string() };
    (@usage) => { "".to_string() };
}

#[derive(Serialize, Deserialize)]
struct Note {
    title: String,
    body: String,
    id: String,
    created_at: i64,
    edited_at: i64,
    tags: Vec<String>,
}

struct Command {
    func: fn(String, Vec<String>),
    usage: String,
}

#[derive(Serialize, Deserialize)]
struct Notes(pub Vec<Note>);

fn main() {
    let args: Vec<String> = env::args().collect();
    let commands = commands! {
        "init"   => cmd_init,
        "list"   => cmd_list,
        "new"    => cmd_new("id"),
        "edit"   => cmd_edit("id"),
        "delete" => cmd_delete("id"),
        "search" => cmd_search("query"),
        "info"   => cmd_info("id"),
        "about"  => cmd_about(""),
    };
    if args.len() <= 2 {
        eprintln!(
            "❌ Not enough arguments, you need at least 2. (you supplied {})",
            args.len() - 1
        );
        eprintln!("RSNote Usage: rsnote <file> <cmd> [opts]");
        eprintln!("Available commands:");
        for (name, command) in commands {
            eprintln!("    {:<8} {}", name, command.usage);
        }
        exit(1);
    }
    let filename = args
        .get(1)
        .expect("No filename (weird, this should NEVER happen)");
    let cmd_name = args.get(2).expect("❌ No command provided.");
    let command = match commands.get(cmd_name) {
        Some(c) => c,
        None => {
            eprintln!("❌ Unknown command: {}", cmd_name);
            eprintln!("Available commands:");
            for (name, c) in commands {
                eprintln!("    {:<8} {}", name, c.usage);
            }
            exit(1);
        }
    };
    (command.func)((&filename).to_string(), (&args[3..]).to_vec());
}
