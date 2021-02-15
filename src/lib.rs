/*********************************************************************************
* Copyright (c) 2018,2021 ADLINK Technology Inc.
*
* This program and the accompanying materials are made available under the
* terms of the Eclipse Public License 2.0 which is available at
* http://www.eclipse.org/legal/epl-2.0, or the Apache Software License 2.0
* which is available at https://www.apache.org/licenses/LICENSE-2.0.
*
* SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
* Contributors:
*   ADLINK fog05 team, <fog05@adlink-labs.tech>
*********************************************************************************/

use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pymodule};

mod im;
use im::*;

#[pymodule]
fn fog05(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(im))?;

    // force addition of "zenoh.net" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['fog05.im'] = im
        ",
        None,
        Some(m.dict()),
    )?;

    Ok(())
}

create_exception!(fog05, FError, exceptions::PyException);

fn to_pyerr(err: fog05_sdk::fresult::FError) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}

fn zrpc_to_pyerr(err: zrpc::zrpcresult::ZRPCError) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}

#[cfg(feature = "cbor")]
fn cbor_to_pyerr(err: serde_cbor::Error) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}

#[cfg(feature = "json")]
fn json_to_pyerr(err: serde_json::Error) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}

#[cfg(feature = "yaml")]
fn yaml_to_pyerr(err: serde_yaml::Error) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}

#[cfg(any(feature = "yaml", feature = "json"))]
fn utf8_to_pyerr(err: std::str::Utf8Error) -> PyErr {
    PyErr::new::<FError, _>(err.to_string())
}
