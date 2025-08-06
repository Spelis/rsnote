use crate::types;
use bincode::{self, config::standard};
use std::process::Command;
use std::{error::Error, fs};

pub fn read_notefile(file: &str) -> Result<types::Notes, Box<dyn Error>> {
	let bytes = fs::read(file)?;
	let config = standard();
	let notes: types::Notes =
		bincode::serde::decode_from_slice(&bytes, config).map(|(notes, _len)| notes)?; // decode returns (T, bytes_read)
	Ok(notes)
}

pub fn write_notefile(file: &str, content: types::Notes) -> Result<(), Box<dyn Error>> {
	let config = standard();
	let encoded = bincode::serde::encode_to_vec(content, config)?;
	fs::write(file, encoded)?;
	Ok(())
}

pub fn get_note(file: &str, id: String) -> Result<types::Note, Box<dyn std::error::Error>> {
	let notefile = read_notefile(&file).unwrap();
	notefile
		.0
		.iter()
		.find(|n| n.id == id)
		.cloned()
		.ok_or_else(|| "Note not found".into())
}

pub fn note_exists(file: &str, id: &str) -> bool {
	get_note(file, id.to_string()).is_ok()
}

/// Open a file with a non-blocking text editor
pub fn open_editor(path: &std::path::Path) -> std::io::Result<()> {
	let editors = ["nvim", "vim", "nano", "vi", "edit"];

	for editor in editors {
		if which::which(editor).is_ok() {
			Command::new(editor).arg(path).status()?;
			return Ok(());
		}
	}
	Err(std::io::Error::new(
		std::io::ErrorKind::NotFound,
		"No suitable editor found in $PATH.",
	))
}

pub fn truncate(text: String, length: usize) -> String {
	let s: &str = &text;
	if s.len() > length {
		let mut ret = s[0..length - 3].trim_end().to_string();
		ret.push_str("...");
		return ret;
	} else {
		return s.to_string();
	}
}
