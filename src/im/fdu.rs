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

use fog05_sdk::im::fdu;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::{cbor_to_pyerr, json_to_pyerr, utf8_to_pyerr, yaml_to_pyerr};

#[pyclass]
#[derive(Clone, Debug)]
pub struct FduDescriptor {
    pub d: fdu::FDUDescriptor,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct FduRecord {
    pub r: fdu::FDURecord,
}

#[pymodule]
pub fn fdu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FduDescriptor>()?;
    m.add_class::<FduRecord>()?;

    Ok(())
}

#[pymethods]
impl FduDescriptor {
    #[cfg(feature = "cbor")]
    fn serialize(&self) -> PyResult<Vec<u8>> {
        serde_cbor::to_vec(&self.d).map_err(cbor_to_pyerr)
    }

    #[cfg(feature = "cbor")]
    #[staticmethod]
    fn deserialize(raw_data: &[u8]) -> PyResult<FduDescriptor> {
        let d = serde_cbor::from_slice::<fdu::FDUDescriptor>(raw_data).map_err(cbor_to_pyerr)?;
        Ok(FduDescriptor { d })
    }

    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.d)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<FduDescriptor> {
        let d = serde_json::from_str::<fdu::FDUDescriptor>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(FduDescriptor { d })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.d).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<FduDescriptor> {
        let d = serde_yaml::from_str::<fdu::FDUDescriptor>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(FduDescriptor { d })
    }
}

#[pyproto]
impl PyObjectProtocol for FduDescriptor {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.d))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

impl pyo3::conversion::ToPyObject for FduDescriptor {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl FduRecord {
    #[cfg(feature = "cbor")]
    fn serialize(&self) -> PyResult<Vec<u8>> {
        serde_cbor::to_vec(&self.r).map_err(cbor_to_pyerr)
    }

    #[cfg(feature = "cbor")]
    #[staticmethod]
    fn deserialize(raw_data: &[u8]) -> PyResult<FduRecord> {
        let r = serde_cbor::from_slice::<fdu::FDURecord>(raw_data).map_err(cbor_to_pyerr)?;
        Ok(FduRecord { r })
    }

    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.r)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<FduRecord> {
        let r = serde_json::from_str::<fdu::FDURecord>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(FduRecord { r })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.r).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<FduRecord> {
        let r = serde_yaml::from_str::<fdu::FDURecord>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(FduRecord { r })
    }
}

#[pyproto]
impl PyObjectProtocol for FduRecord {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.r))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

impl pyo3::conversion::ToPyObject for FduRecord {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}
