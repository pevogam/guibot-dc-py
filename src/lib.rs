// Copyright 2018, 2019, 2020 Michael Sanders
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT License <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// #![feature(specialization, const_fn)]

extern crate autopilot;
extern crate either;
extern crate image;
extern crate pyo3;

pub mod alert;
pub mod bitmap;
pub mod color;
mod internal;
pub mod key;
pub mod mouse;
pub mod screen;

use pyo3::prelude::*;

// ⚠  Warning: Couldn't find the symbol `PyInit_autopy` in the native library. Python will fail to import this module. If you're using pyo3, check that `#[pymodule]` uses `autopy` as module name
#[pymodule]
fn autopy(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
