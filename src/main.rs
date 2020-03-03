/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
#![allow(unused)]
use cpal::traits::{DeviceTrait, HostTrait};
use deepspeech::{Metadata, Model};

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("Unable to get default input device");

    let config = device
        .default_input_config()
        .expect("Unable to get default config for input device");

    println!("Hello, world!");
}
