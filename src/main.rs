use book_tldr::Listing;
use book_tldr::get_files;
use clap::Parser;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
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
            check_match(a.chapter, &args.chapter)
                && check_match(a.description, &args.description)
                && check_match(a.listing, &args.listing)
                && check_match(a.status, &args.status)
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
        let listing_file = results[0].listing.to_owned() + ".md";
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
fn check_match(source: &str, query: &str) -> bool {
    if query == "" || source.contains(query) {
        return true;
    }
    let matcher = SkimMatcherV2::default();
    let test = matcher.fuzzy_indices(source, query);
    if test != None {
        let (score, _indices) = test.unwrap();
        return score >= 50;
    } else {
        return false;
    }
}

// print listing info
fn print_info(file: &Listing) {
    println!("chapter: {}", file.chapter);
    println!("description: {}", file.description);
    println!("listing: {}, status: {}\n", file.listing, file.status);
}
