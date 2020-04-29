extern crate komari;
use structopt::StructOpt;
use serde_json::Value;
use crate::komari::pulls;
use crate::komari::view;

#[derive(StructOpt)]
struct Cli {
    owner: String,
    repo: String,
    since: i64,
}

fn main() {
    let args = Cli::from_args();
    println!("repository is {}/{}", args.owner, args.repo);
    println!("select newer than #{}", args.since);

    match pulls::list(&args.owner, &args.repo) {
        Ok(json) => {
            if let Some(json) = json {
                if let Some(array) = json.as_array() {
                    let res: Vec<Value> = pulls::select(&array, &args.since);
                    view::display(&res);
                }

            }
        },
        Err(e) => panic!(e)
    }
}
