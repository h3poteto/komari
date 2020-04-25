use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    owner: String,
    repo: String,
}

fn main() {
    let args = Cli::from_args();
    println!("repository is {}/{}", args.owner, args.repo);
}
