use std::fs::{read_dir, File};
use std::path::PathBuf;

use clap::Parser;

use gpx::{Gpx, Route};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    r#in: PathBuf,
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let mut res = Gpx::default();
    res.routes.push(Route::default());

    for file in read_dir(cli.r#in).unwrap() {
        let file = file.unwrap().path();
        println!("{file:?}");
        let file = File::open(file).unwrap();
        let mut g = gpx::read(file).unwrap();

        res.version = g.version;
        res.creator = g.creator;
        res.metadata = g.metadata;

        res.routes[0].points.append(&mut g.routes[0].points);
    }

    let file = File::create(cli.out).unwrap();

    gpx::write(&res, file).unwrap();
}
