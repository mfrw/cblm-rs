use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

mod cblmariner;
use cblmariner::Repository;

fn main() -> std::io::Result<()> {
    let str = fs::read_to_string("specs.json")?;
    let rp: Repository = serde_json::from_str(&str)?;
    rp.repo
        .into_iter()
        .filter(|p| p.dependency().contains(&"golang"))
        .filter_map(|p| p.spec_path)
        .sorted()
        .dedup()
        .par_bridge()
        .for_each(|r| println!("{:?}", r.rsplit_once("/").unwrap().1));
    Ok(())
}
