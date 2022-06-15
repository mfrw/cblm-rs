use std::collections::HashSet;
use std::fs;

use clap::Parser;
use itertools::Itertools;
use rayon::prelude::*;

mod cblmariner;
use cblmariner::Repository;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, value_parser)]
    base_dir: String,

    #[clap(short, long, value_parser)]
    specs_json_file: String,

    #[clap(short, long, value_parser, default_value = ".spec")]
    ends_with: String,

    #[clap(short, long, value_parser)]
    needle: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let it = get_files_ending_with(&args.base_dir, &args.ends_with).par_bridge();

    let str = fs::read_to_string(&args.specs_json_file)?;
    let rp: Repository = serde_json::from_str(&str)?;
    let st = rp
        .repo
        .into_iter()
        .filter(|p| p.dependency().find(|&d| d == &args.needle).is_some())
        .filter_map(|p| p.spec_path)
        .sorted()
        .dedup()
        .par_bridge()
        .map(|p| p.rsplit_once("/").unwrap().1.to_string())
        .collect::<HashSet<_>>();

    it.filter(|p| {
        let name = p.rsplit_once("/").unwrap().1;
        st.contains(name)
    })
    .for_each(|p| println!("{p}"));

    Ok(())
}

fn get_files_ending_with<'a>(base: &'a str, suffix: &'a str) -> impl Iterator<Item = String> + 'a {
    walkdir::WalkDir::new(base)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .map(|e| e.path().display().to_string())
        .filter(move |s| s.ends_with(suffix))
}
