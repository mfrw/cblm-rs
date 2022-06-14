use itertools::Itertools;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    pub provides: PackageVer,
    pub srpm_path: Option<String>,
    pub rpm_path: Option<String>,
    pub source_dir: Option<String>,
    pub spec_path: Option<String>,
    pub architecture: Option<String>,
    pub requires: Option<Vec<PackageVer>>,
    pub build_requires: Option<Vec<PackageVer>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PackageVer {
    pub name: String,
    pub version: String,
    pub condition: String,
    pub s_version: String,
    pub s_condition: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Repository {
    pub repo: Vec<Package>,
}

impl Package {
    pub fn requires(&self) -> impl Iterator<Item = &str> + '_ {
        self.requires
            .iter()
            .flat_map(|v| v.iter().map(|p| p.name.as_str()))
    }

    pub fn build_requires(&self) -> impl Iterator<Item = &str> + '_ {
        self.build_requires
            .iter()
            .flat_map(|v| v.iter().map(|p| p.name.as_str()))
    }

    pub fn dependency(&self) -> impl Iterator<Item = &str> + '_ {
        self.build_requires()
            .into_iter()
            .chain(self.requires().into_iter())
            .sorted()
            .dedup()
    }
}
