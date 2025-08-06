use tabled::{Table, settings::Style};

use crate::types::{self};

pub fn init(file: String) {
	let notefile: types::Notes = types::Notes { 0: vec![] };
	_ = crate::helpers::write_notefile(&file, notefile);
	println!("Initialized Notefile at '{}'", file);
}
pub fn list(file: String) {
	let notefile = crate::helpers::read_notefile(&file).unwrap();
	let mut table = Table::new(notefile.0);
	table.with(Style::rounded());
	println!("{}", table);
}

pub fn search(file: String, query: Vec<String>) {
	let notefile = crate::helpers::read_notefile(&file).unwrap();
	let mut result: Vec<types::Note> = Vec::new();
	for note in notefile.0 {
		for q in query.clone().into_iter() {
			if note.title.contains(&q) {
				result.push(note);
				break;
			}
		}
	}
	let mut table = Table::new(result);

	table.with(Style::rounded());
	println!("{}", table);
}
pub fn search_tag(file: String, tag: Vec<String>) {
	let notefile = crate::helpers::read_notefile(&file).unwrap();
	let mut result: Vec<types::Note> = Vec::new();
	for note in notefile.0 {
		for q in tag.clone().into_iter() {
			if note.tags.contains(&q) {
				result.push(note);
				break;
			}
		}
	}
	let mut table = Table::new(result);

	table.with(Style::rounded());
	println!("{}", table);
}

pub mod mutate {
	use tabled::settings::object::Rows;
	use tabled::settings::{Modify, Padding, Style};
	use tabled::{Table, Tabled};

	use crate::helpers::write_notefile;
	use crate::{
		helpers::{self, open_editor},
		types::{self},
	};
	use std::fs;
	use std::io::Write;

	pub fn new(file: String, id: String) {
		if helpers::note_exists(&file, id.as_str()) {
			println!("Note with id {} already exists.", id);
			return;
		}

		let mut tmp = tempfile::NamedTempFile::new().expect("Failed to create tempfile.");
		writeln!(tmp, "<title>\n<body>").expect("Failed to write.");
		open_editor(tmp.path()).expect("Failed to open tempfile.");

		let content = fs::read_to_string(tmp).expect("Failed to read tempfile.");
		let title = content.lines().next().unwrap_or("Untitled");
		let body = content.lines().skip(1).next().unwrap_or("No Body.");
		let now = chrono::Local::now().timestamp();
		let mut notes = helpers::read_notefile(&file).expect("Invalid note file.");

		notes.0.push(crate::types::Note {
			id: id.clone(),
			title: (*title).to_string(),
			body: (*body).to_string(),
			edited: 0,
			created: now,
			tags: vec![],
		});
		_ = helpers::write_notefile(&file, notes);
		println!("Successfully created and saved \"{}\"", id);
	}
	pub fn edit(file: String, id: String) {
		let oldnote = helpers::get_note(&file, id.clone()).unwrap();
		let mut tmp = tempfile::NamedTempFile::new().expect("Failed to create tempfile.");
		writeln!(tmp, "{}\n{}", oldnote.title, oldnote.body).expect("Failed to write.");
		open_editor(tmp.path()).expect("Failed to open tempfile.");

		let content = fs::read_to_string(tmp).expect("Failed to read tempfile.");
		let title = content.lines().next().unwrap_or("Untitled");
		let body = content.lines().skip(1).next().unwrap_or("No Body.");
		let now = chrono::Local::now().timestamp();

		let mut notes = helpers::read_notefile(&file).expect("Invalid note file.");
		if let Some(note) = notes.0.iter_mut().find(|n| n.id == id) {
			*note = crate::types::Note {
				id: id.clone(),
				title: title.to_string(),
				body: body.to_string(),
				edited: now,
				created: note.created,
				tags: note.tags.clone(),
			};
		}
		_ = helpers::write_notefile(&file, notes);
		println!("Successfully created and saved \"{}\"", id);
	}
	pub fn delete(file: String, id: String) {
		let mut notes = helpers::read_notefile(&file).expect("Invalid note file.");
		let index: usize = notes.0.clone().iter().position(|n| n.id == id).unwrap();
		notes.0.remove(index);
		println!("Deleted.")
	}
	#[derive(Tabled)]
	struct Info {
		key: &'static str,
		value: String,
	}
	pub fn info(file: String, id: String) {
		let note = helpers::get_note(&file, id).expect("Note not found.");
		let mut table = Table::new(vec![
			Info {
				key: "ID",
				value: note.id,
			},
			Info {
				key: "Title",
				value: note.title,
			},
			Info {
				key: "Body",
				value: note.body,
			},
			Info {
				key: "Created",
				value: types::display_ts(&note.created),
			},
			Info {
				key: "Edited",
				value: types::display_ts(&note.edited),
			},
			Info {
				key: "Tags",
				value: note.tags.join(", "),
			},
		]);
		table
			.with(Style::rounded())
			.with(Modify::new(Rows::new(..)).with(Padding::new(1, 1, 0, 0)));

		println!("{}", table);
	}
	pub fn toggle_tag(file: String, id: String, tag: String) {
		let mut notes = helpers::read_notefile(&file).expect("Invalid note file.");
		let note = notes.0.iter_mut().find(|n| n.id == id).unwrap();
		if let Some(pos) = note.tags.iter().position(|x| *x == tag) {
			note.tags.remove(pos);
			println!("Removed the \"{}\" tag.", tag);
		} else {
			note.tags.push(tag.clone());
			println!("Added the \"{}\" tag.", tag);
		}
		_ = write_notefile(&file, notes);
	}
}

pub mod crypt {
	// pub fn en(file: String, id: String, pass: String) {}
	// pub fn de(file: String, id: String, pass: String) {}
}
