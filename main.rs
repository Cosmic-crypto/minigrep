use argh::FromArgs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Search for a query string in a file
#[derive(FromArgs)]
struct Args {
    /// file to search in
    #[argh(positional)]
    filename: String,

    /// string to search for
    #[argh(positional)]
    query: String,

    /// find all matches (otherwise stop at first)
    #[argh(switch, short = 'f')]
    findall: bool,
}

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();

    println!("Checking for '{}' in file: '{}'...", args.query, args.filename);

    let file: File = File::open(&args.filename)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut found: bool = false;
    let mut queries_found: u32 = 0;

    for (idx, line) in reader.lines().enumerate() {
        let line: String = line?;

        if line.contains(&args.query) {
            found = true;

            if !args.findall {
                println!(
                    "first '{}' found at line {} in file: {}",
                    args.query, idx + 1, args.filename
                );
                break;
            }

             println!(
                "'{}' found at line {} in file: {}",
                args.query, idx + 1, args.filename
            );
            queries_found += 1;
        }
    }

    if !found {
        println!("'{}' was not found in '{}'", args.query, args.filename);
    } else if args.findall {
        println!("\nfound query: '{}', {} times", args.query, queries_found);
    }

    Ok(())
}
