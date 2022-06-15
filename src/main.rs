use std::collections::HashSet;
use std::fs;

use itertools::Itertools;
use rayon::prelude::*;

mod cblmariner;
use cblmariner::Repository;

fn main() -> std::io::Result<()> {
    let it =
        get_files_ending_with("/home/mfrw/mariner-org/CBL-Mariner/SPECS/", ".spec").par_bridge();

    let str =
        fs::read_to_string("/home/mfrw/mariner-org/CBL-Mariner/build/pkg_artifacts/specs.json")?;
    let rp: Repository = serde_json::from_str(&str)?;
    let st = rp
        .repo
        .into_iter()
        .filter(|p| p.dependency().contains(&"golang"))
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
    .for_each(|p| print!("{p} "));

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
