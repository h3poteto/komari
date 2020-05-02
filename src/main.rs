extern crate komari;
use crate::komari::pulls;
use crate::komari::view;
use structopt::StructOpt;

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

    match pulls::Pulls::new(&args.owner, &args.repo) {
        Ok(client) => match client.list_pulls_since(&args.since) {
            Ok(array) => match client.select(&array, &args.since) {
                Ok(res) => {
                    view::display(&res);
                }
                Err(e) => panic!(e),
            },
            Err(e) => panic!(e),
        },
        Err(e) => panic!(e),
    }
}
