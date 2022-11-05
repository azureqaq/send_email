use clap::{crate_authors, crate_name, Command};

fn main() {
    let _app = Command::new(crate_name!())
        .about("a good demo!")
        .author(crate_authors!())
        .build();
}
