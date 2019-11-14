use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::hash::{Hasher, Hash};
use std::path::Path;

use glob::{glob_with, MatchOptions};
use failure::{format_err, ResultExt};
use serde::{Deserialize, Serialize};
// use docopt::Docopt;

pub type Result<T> = ::std::result::Result<T, ::failure::Error>;


#[derive(Deserialize, Serialize, Debug)]
struct RegistryPackage {
    name: String,
    vers: String,
    deps: Vec<RegistryDependency>,
    cksum: String,
    features: BTreeMap<String, Vec<String>>,
    yanked: Option<bool>,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
struct RegistryDependency {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>,
    package: Option<String>
}

impl Hash for RegistryPackage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.vers.hash(state);
        self.cksum.hash(state);
        self.features.hash(state);
    }
}

impl PartialEq for RegistryPackage {
    fn eq(&self, other: &Self) -> bool {
        self.cksum == other.cksum
        && self.name == other.name
        && self.vers == other.vers
        && self.features == other.features
    }
}

impl Eq for RegistryPackage {}

// fn parse_glob

fn collect_packages(path: &str) -> Result<HashSet<RegistryPackage>> {
    let mut packages: HashSet<RegistryPackage> = HashSet::new();
    let glob_str = format!("{}/**/*", path);
    for entry in glob_with(
        &glob_str,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: true,
        }
    )? {
        let entry = entry?;  // gross
        if !entry.is_dir() {
            let file = File::open(&entry)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                // each line is a separate json object
                packages.insert(
                    serde_json::from_str(&line?).context(format!("Bad Json at {:?}", &entry))?
                );
            }
        }
    }
    if packages.len() == 0 {
        Err(format_err!("No packages found at at {}", glob_str))
    } else {
        Ok(packages)
    }
}

fn main_real() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let old_path = std::fs::canonicalize(args.get(1).expect("erg"))?;
    let new_path = std::fs::canonicalize(args.get(2).expect("erg"))?;

    let old = collect_packages(&old_path.to_str().expect("yeah nah"))?;
    let new = collect_packages(&new_path.to_str().expect("yeah nah"))?;
    let only_in_old: HashSet<_> = old.difference(&new).collect();
    let only_in_new: HashSet<_> = new.difference(&old).collect();
    if only_in_old.len() == 0 {
        println!("Success! {:?} contains all {} packages in {:?}, and {} new packages", new_path, old.len(), old_path, only_in_new.len());
        Ok(())
    } else {
        println!("Failure! there are {} packages in that are in {:?} but not in {:?} ", only_in_old.len(), old_path, new_path);
        Err(format_err!("Missing package"))
    }
}

fn main() {
    let retcode = match main_real() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e.as_fail());
            1
        },
    };
    std::process::exit(retcode);
}
