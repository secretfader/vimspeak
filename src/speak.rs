/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
//! Provides the main interface to VimSpeak, including generating
//! required configuration files and running speech-to-text operations
use async_std::task;
use cpal::traits::DeviceTrait;
use deepspeech::Model;
use std::{convert::TryFrom, path::Path};

mod helpers;
mod vad;

/// Supported DeepSpeech model version
const DS_VERSION: &str = "0.6.1";

/// Install required configuration, model, and library files
pub fn install(c: &Config) -> Result<(), Error> {
    task::block_on(async {
        let _model_path = c.model_path.to_owned().into_os_string();
    });
    Ok(())
}

/// Run speech-to-text operation with pre-installed model and library
pub fn run(c: &Config) -> Result<(), Error> {
    let host = cpal::default_host();
    let input = c.device.to_owned();
    let device = helpers::select_input_device(&host, input)?;

    let input_config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(c.sample_rate.to_owned()),
    };

    log::debug!("Input config: {:?}", &input_config);

    let model_path = Path::new(&c.model_path);
    let mut m = Model::load_from_files(&model_path.join("output_graph.pbmm"), c.beam_width)
        .expect("Unable to initialize model");
    let _stt = m.create_stream().expect("unable to create stream");

    let mut vad = webrtc_vad::Vad::new();
    vad.set_sample_rate(webrtc_vad::SampleRate::try_from(c.sample_rate as i32).unwrap());

    let data_fn = move |_data: &[f32]| {};

    let err_fn = |mut _e| {};

    let _input = device
        .build_input_stream(&input_config, data_fn, err_fn)
        .map_err(|_| Error::DeviceIO)?;

    Ok(())
}

/// Errors that may be returned during setup or speech-to-text
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Config not available")]
    NoConfig,
    #[error("Requested (or default) input device unavailable")]
    NoInputDevice,
    #[error("Device I/O error")]
    DeviceIO,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    model_path: std::path::PathBuf,
    beam_width: u16,
    sample_rate: u32,
    device: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let config_path = dirs::config_dir().expect("Unable to locate config dir");
        Config {
            model_path: config_path.join(format!("vimspeak/deepspeech-{}-models", DS_VERSION)),
            beam_width: 1,
            sample_rate: 16_000,
            device: None,
        }
    }
}
