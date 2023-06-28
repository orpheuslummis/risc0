// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
risc0_zkvm::guest::entry!(main);

use orpheus_core::{Output, Vote, Votes};
use risc0_zkvm::guest::env;

pub fn main() {
    let votes: Votes = env::read();

    let mut avg = 0;
    for vote in &votes {
        avg += vote;
    }
    avg /= votes.len() as Vote;

    let out = Output { result: avg };
    env::commit(&out);
}
