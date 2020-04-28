extern crate komari;
use structopt::StructOpt;
use crate::komari::pulls;

#[derive(StructOpt)]
struct Cli {
    owner: String,
    repo: String,
}

fn main() {
    let args = Cli::from_args();
    println!("repository is {}/{}", args.owner, args.repo);

    match pulls::list(&args.owner, &args.repo) {
        Ok(json) => {
            if let Some(json) = json {
                println!("{}", json)
            }
        },
        Err(e) => panic!(e)
    }
}
