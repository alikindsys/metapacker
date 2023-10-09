pub enum Source {
    Modrinth(ModrinthSource),
    Github(GithubSource),
    Https // Raw Https source. Requires TLS to work.
}

pub enum ModrinthSource {
    Mod,
    ModPack
}

pub enum GithubSource {
    Raw, // raw.github.com, used for getting config files and other things.
    Release // A github release
}