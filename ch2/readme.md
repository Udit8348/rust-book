# TOML
"Tom's Obvious Minimal Language"

- [] are section headings
- package section: the following statements are configuring a package
- dependencies section: list other dependencies referred to as crates
- crate's version follows Semantic Versioning
- semantic versioning makes sure you get the latest patches for your version, but guarantee compatibility so your code will compile
- when we include a crate it will be a library crate not a binary create which we create when running cargo build/run etc
- cargo fetches the latest versions of the deps you have specified from the registry
- the registry is a copy of the data on Crates.io which is where people post OS Rust projects
- after downloading the crates, they are compiled for the first time
- if the dependencies are not changed, then they are no recompiled
- when you build a project for the first time, cargo saves the working versions of deps in .lock file
- if there is a breaking update, then it will not be added to the project, thanks to .lock file
