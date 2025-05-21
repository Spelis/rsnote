use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::Command;
use std::process::exit;

fn open() {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    Command::new(editor)
        .arg("/tmp/rsnote.txt")
        .status()
        .expect("Failed to open editor");
}

fn index_of<T: PartialEq>(list: &[T], target: &T) -> isize {
    match list.iter().position(|x| x == target) {
        Some(idx) => idx as isize,
        None => -1,
    }
}

fn search_notes<'a>(notes: &'a [String], query: &str) -> Vec<(usize, &'a String)> {
    notes
        .iter()
        .enumerate()
        .filter(|(_, note)| note.contains(query))
        .collect()
}

// File format:
//
// "filetype": "rsnote-vault" NOTE: not encrypted, used as "header".
// "notes": [
//  "this is a text body, cry about it bitch." NOTE: each string is its own note, 0-indexed the title is considered until the first newline
//  OR like the first 25 characters
// ]

// TODO: for editing (and creating new) notes, do something like git where it creates a tempfile
// and opens in a text editor the user chooses.

#[derive(Serialize, Deserialize)]
struct Notes {
    filetype: String,
    notes: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("RSNote Usage: rsnote <notefile> <cmd> [opts]");
        eprintln!("Commands:\ninit, list, new, edit, delete, search, full");
        exit(1);
    }

    if args[2] == "init" {
        let filename = args.get(1).expect("Filename required");

        fs::File::create(&filename).expect("Failure. \n");

        let data = Notes {
            filetype: String::from("rsnote-vault"), // Required!
            notes: vec![],
        };

        fs::write(&filename, serde_json::to_string_pretty(&data).unwrap())
            .expect("Failed to write");

        println!("Successfully initialized Note with name {}", filename);
    }
    if args[2] == "list" {
        let filename = args.get(1).expect("Filename required");

        let content = fs::read_to_string(filename).expect("Failed to read the note file.");
        let notes: Notes = serde_json::from_str(&content).unwrap();

        for note in &notes.notes {
            let (title, _body) = note.split_once("\n").unwrap_or((note, ""));
            println!("Note {}: {}...", index_of(&notes.notes, note), title);
        }
    }
    if args[2] == "new" {
        let filename = args.get(1).expect("Filename required");

        let content = fs::read_to_string(&filename).expect("Failed to open the note file.");
        let mut notes: Notes = serde_json::from_str(&content).unwrap();

        fs::write("/tmp/rsnote.txt", String::new()).expect("Failed to create tempfile.");
        open();
        let content =
            fs::read_to_string("/tmp/rsnote.txt").expect("Failed to read tempfile content.");

        notes.notes.push(content);

        fs::write(&filename, serde_json::to_string_pretty(&notes).unwrap())
            .expect("Failed to write");
        println!("Successfully created the note");
    }
    if args[2] == "edit" {
        let filename = args.get(1).expect("Filename required");
        let index = args
            .get(3)
            .expect("Index required")
            .parse::<usize>()
            .expect("Index must be a number");

        let content = fs::read_to_string(&filename).expect("Failed to open the note file.");
        let mut notes: Notes = serde_json::from_str(&content).unwrap();

        fs::write("/tmp/rsnote.txt", &notes.notes[index]).expect("Failed to create tempfile.");
        open();
        let content =
            fs::read_to_string("/tmp/rsnote.txt").expect("Failed to read tempfile content.");

        notes.notes[index] = content;

        fs::write(&filename, serde_json::to_string_pretty(&notes).unwrap())
            .expect("Failed to write");
        println!("Successfully edited the note");
    }
    if args[2] == "delete" {
        let filename = args.get(1).expect("Filename required");
        let index = args
            .get(3)
            .expect("Index required")
            .parse::<usize>()
            .expect("Index must be a number");

        let content = fs::read_to_string(&filename).expect("Failed to open the note file.");
        let mut notes: Notes = serde_json::from_str(&content).unwrap();

        notes.notes.remove(index);

        fs::write(&filename, serde_json::to_string_pretty(&notes).unwrap())
            .expect("Failed to write");
        println!("Successfully deleted the note");
    }
    if args[2] == "search" {
        let filename = args.get(1).expect("Filename required");
        let keyword = args.get(3).expect("Keyword(s) required.");

        let content = fs::read_to_string(&filename).expect("Failed to open the note file.");
        let notes: Notes = serde_json::from_str(&content).unwrap();

        let result = search_notes(&notes.notes, keyword);

        println!(
            "Search for \"{}\" returned {} results:",
            keyword,
            result.len()
        );

        for (i, note) in result {
            let (title, _body) = note.split_once("\n").unwrap_or((note, ""));
            println!("Note {}: {}...", i, title);
        }
    }
    if args[2] == "read" {
        let filename = args.get(1).expect("Filename required");
        let index = args
            .get(3)
            .expect("Index required")
            .parse::<usize>()
            .expect("Index must be a number");

        let content = fs::read_to_string(&filename).expect("Failed to open the note file.");
        let notes: Notes = serde_json::from_str(&content).unwrap();

        println!("{}", &notes.notes[index]);
    }
}
