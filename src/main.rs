extern crate komari;
use structopt::StructOpt;
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

    let client = pulls::Pulls::new(&args.owner, &args.repo);

    match client.list() {
        Ok(json) => {
            if let Some(array) = json.as_array() {
                match client.select(&array, &args.since) {
                    Ok(res) => {
                        view::display(&res);
                    },
                    Err(e) => panic!(e)
                }
            }

        },
        Err(e) => panic!(e)
    }
}
