use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum Source {
    Modrinth {
        kind: ModrinthSource,
        project_id: String,
        project_slug: String,
        project_version: String
    },
    Github {
        kind: GithubSource,
        author: String,
        repo: String
    },
    Https {
        file: String,
        uri: String
    }// Raw Https source. Requires TLS to work.
}

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum ModrinthSource {
    Mod,
    ModPack
}

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum GithubSource {
    Raw { path: String, revision: String }, // raw.github.com, used for getting config files and other things.
    Release { tag: String, file: String } // A github release
}