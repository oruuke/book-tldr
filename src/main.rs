use book_tldr::File;
use book_tldr::get_files;
use clap::Parser;
use std::error::Error;
use std::fs;
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

// start program and error if needed
fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("oopsie: {e}");
        process::exit(1);
    }
}

// run program logic
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // get files from library
    let files = get_files();

    // iterate over files and filter results across all fields
    let results: Vec<&'static File> = files
        .iter()
        .filter(|a| {
            a.chapter.contains(&args.chapter)
                && a.description.contains(&args.description)
                && a.listing.contains(&args.listing)
                && a.status.contains(&args.status)
        })
        .map(|a| a)
        .collect();

    // print file details for each result if multiple
    if results.len() > 1 {
        println!("all matched listings:\n");
        for result in results {
            print_info(result);
        }
    // print all listing code if just one result
    } else if results.len() == 1 {
        let contents = fs::read_to_string(results[0].listing.to_string() + ".md");
        println!("full listing:\n");
        print_info(results[0]);

        if let Ok(line) = contents {
            println!("{line}");
        }
    } else {
        println!("none found!");
    }

    Ok(())
}

fn print_info(file: &File) {
    println!("chapter: {}", file.chapter);
    println!("description: {}", file.description);
    println!("listing: {}, status: {}\n", file.listing, file.status);
}
