/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
mod speak;
use structopt::StructOpt;

fn main() -> Result<(), speak::Error> {
    let c: speak::Config = confy::load("vimspeak").map_err(|_| speak::Error::NoConfig)?;
    match CLI::from_args() {
        CLI::Install => speak::install(&c),
        CLI::Run => speak::run(&c),
    }
}

/// an experiment in hands-free editing
#[derive(StructOpt)]
enum CLI {
    /// Create required configuration
    Install,
    /// Run speech-to-text
    Run,
}
