use std::fs;

mod cblmariner;
use cblmariner::Repository;

fn main() -> std::io::Result<()> {
    let str = fs::read_to_string("specs.json")?;
    let rp: Repository = serde_json::from_str(&str)?;
    rp.repo
        .into_iter()
        .skip(300)
        .take(2)
        .for_each(|r| println!("{:?}", r));
    Ok(())
}
