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
