use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum Source {
    Modrinth(ModrinthSource),
    Github(GithubSource),
    Https // Raw Https source. Requires TLS to work.
}

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum ModrinthSource {
    Mod,
    ModPack
}

#[derive(Deserialize, Serialize, PartialOrd, PartialEq, Debug)]
pub enum GithubSource {
    Raw, // raw.github.com, used for getting config files and other things.
    Release // A github release
}