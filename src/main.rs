use book_tldr::Listing;
use book_tldr::get_files;
use clap::Parser;
use include_dir::{Dir, include_dir};
use std::error::Error;
use std::process;

// get args from user, defaulting to universally matching string
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "")]
    chapter: String,
    #[clap(short, long, default_value = "")]
    description: String,
    #[clap(short, long, default_value = "")]
    listing: String,
    #[clap(short, long, default_value = "desired")]
    status: String,
}

static ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/listings");

// start program and error if needed
fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("oopsie! error running: {e}");
        process::exit(1);
    }
}

// run program logic
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // get files from library
    let files = get_files();

    // iterate over files and filter results across all fields
    let results: Vec<&'static Listing> = files
        .iter()
        .filter(|a| {
            split_match(a.chapter, &args.chapter) > 0
                && split_match(a.description, &args.description) > 0
                && split_match(a.listing, &args.listing) > 0
                && split_match(a.status, &args.status) > 0
        })
        .map(|a| a)
        .collect();

    // print listing details for each result if multiple
    if results.len() > 1 {
        println!("all matched listings:\n");
        for result in results {
            print_info(result);
        }
    // print all listing code if just one result
    } else if results.len() == 1 {
        println!("full listing:\n");
        print_info(results[0]);

        // read and print with embedded listing file
        let listing = results[0].listing;
        let listing_file = listing[1..listing.len() - 1].to_owned() + ".md";
        if let Some(s) = read_text(&listing_file) {
            println!("{s}")
        } else {
            eprintln!("oopsie! could not find file");
        }
    } else {
        println!("none found!");
    }

    Ok(())
}

// read embedded file
fn read_text(name: &str) -> Option<&'static str> {
    ASSETS.get_file(name)?.contents_utf8()
}

// TODO: replace fuzzy match wit split match
fn split_match<'a>(source: &'a str, query: &'a str) -> u32 {
    if query.len() == 0 {
        return 1;
    }
    let bytes = query.as_bytes();
    let mut words: Vec<&str> = Vec::new();
    let mut remaining: &str = &query;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let diff = query.len() - remaining.len();
            words.push(&remaining[..i - diff].trim());
            remaining = &query[i..];
        }
        if i == query.len() - 1 {
            words.push(remaining);
        }
    }

    let mut score = 0;
    for word in &words {
        if source.contains(word) {
            score = score + 1;
        } else {
            return 0;
        }
    }

    score
}

// print listing info
fn print_info(file: &Listing) {
    println!("chapter: {}", file.chapter);
    println!("description: {}", file.description);
    println!("listing: {}, status: {}\n", file.listing, file.status);
}
