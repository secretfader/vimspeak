/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
//! Helper functions and other convenience interfaces
use cpal::traits::{DeviceTrait, HostTrait};

/// Query for an (optional) input device, or select the default input for
/// a provided host
pub(crate) fn select_input_device(
    host: &cpal::Host,
    device: Option<String>,
) -> Result<cpal::Device, super::Error> {
    match device {
        Some(d) => {
            let mut input = host
                .input_devices()
                .map_err(|_| super::Error::NoInputDevice)?
                .filter(|i| {
                    if let Ok(name) = i.name() {
                        name == d
                    } else {
                        false
                    }
                });

            let detected = input.next();
            let count = input.count();
            if count == 0 {
                return Err(super::Error::NoInputDevice);
            }

            if let Some(device) = detected {
                Ok(device)
            } else {
                Err(super::Error::NoInputDevice)
            }
        }
        None => {
            if let Some(device) = host.default_input_device() {
                Ok(device)
            } else {
                Err(super::Error::NoInputDevice)
            }
        }
    }
}

#[test]
fn test_select_default_input_device() {
    let host = cpal::default_host();
    let _ = select_input_device(&host, None).unwrap();
}
