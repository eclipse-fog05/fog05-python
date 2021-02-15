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

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod fdu;
use fdu::*;

mod entity;
use entity::*;

#[pymodule]
fn im(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(fdu))?;
    py.run(
        "\
import sys
sys.modules['fog05.im.fdu'] = fdu
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_wrapped(wrap_pymodule!(entity))?;
    py.run(
        "\
import sys
sys.modules['fog05.im.entity'] = entity
        ",
        None,
        Some(m.dict()),
    )?;

    Ok(())
}
