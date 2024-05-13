// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use cli::CliArguments;

mod cli;
mod polyproto;

fn main() {
    let args = CliArguments::parse();
    println!("{:?}", args);
}
