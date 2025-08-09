use clap::{Parser, Subcommand};
use sodiumoxide::init;

mod commands;
mod helpers;
mod types;

#[derive(Parser, Debug)]
struct Args {
	file: String,
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
	/// Initialize notefile
	Init,
	/// List notes
	List,
	/// Create a new note with ID
	New { id: String },
	/// Edit an existing note by ID
	Edit { id: String },
	/// Delete a note by ID
	Delete { id: String },
	/// Search through notes by *QUERY
	Search { query: Vec<String> },
	/// Show info about note at ID
	Info { id: String },
	/// Toggle a TAG for note ID
	ToggleTag { id: String, tag: String },
	/// Search through notes by *TAG
	SearchTag { tag: Vec<String> },
}

fn main() {
	let start = chrono::Local::now();
	let args: Args = Args::parse();
	match args.command {
		Command::Init => commands::init(args.file),
		Command::List => commands::list(args.file),
		Command::New { id } => commands::mutate::new(args.file, id),
		Command::Edit { id } => commands::mutate::edit(args.file, id),
		Command::Delete { id } => commands::mutate::delete(args.file, id),
		Command::Search { query } => commands::search(args.file, query),
		Command::Info { id } => commands::mutate::info(args.file, id),
		Command::ToggleTag { id, tag } => commands::mutate::toggle_tag(args.file, id, tag),
		Command::SearchTag { tag } => commands::search_tag(args.file, tag),
	}
	let end = chrono::Local::now();
	println!(
		"Executed in {} seconds.",
		(end - start).num_microseconds().unwrap_or(0) as f64 / 1_000_000.0
	);
}
