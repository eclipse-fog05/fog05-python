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

use fog05_sdk::im::entity;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::{cbor_to_pyerr, json_to_pyerr, utf8_to_pyerr, yaml_to_pyerr};

#[pyclass]
#[derive(Clone, Debug)]
pub struct EntityDescriptor {
    pub d: entity::EntityDescriptor,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct EntityRecord {
    pub r: entity::EntityRecord,
}

#[pymodule]
pub fn entity(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EntityDescriptor>()?;
    m.add_class::<EntityRecord>()?;

    Ok(())
}

#[pymethods]
impl EntityDescriptor {
    #[cfg(feature = "cbor")]
    fn serialize(&self) -> PyResult<Vec<u8>> {
        serde_cbor::to_vec(&self.d).map_err(cbor_to_pyerr)
    }

    #[cfg(feature = "cbor")]
    #[staticmethod]
    fn deserialize(raw_data: &[u8]) -> PyResult<EntityDescriptor> {
        let d =
            serde_cbor::from_slice::<entity::EntityDescriptor>(raw_data).map_err(cbor_to_pyerr)?;
        Ok(EntityDescriptor { d })
    }

    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.d)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<EntityDescriptor> {
        let d = serde_json::from_str::<entity::EntityDescriptor>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(EntityDescriptor { d })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.d).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<EntityDescriptor> {
        let d = serde_yaml::from_str::<entity::EntityDescriptor>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(EntityDescriptor { d })
    }
}

#[pyproto]
impl PyObjectProtocol for EntityDescriptor {
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

#[pymethods]
impl EntityRecord {
    #[cfg(feature = "cbor")]
    fn serialize(&self) -> PyResult<Vec<u8>> {
        serde_cbor::to_vec(&self.r).map_err(cbor_to_pyerr)
    }

    #[cfg(feature = "cbor")]
    #[staticmethod]
    fn deserialize(raw_data: &[u8]) -> PyResult<EntityRecord> {
        let r = serde_cbor::from_slice::<entity::EntityRecord>(raw_data).map_err(cbor_to_pyerr)?;
        Ok(EntityRecord { r })
    }

    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.r)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<EntityRecord> {
        let r = serde_json::from_str::<entity::EntityRecord>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(EntityRecord { r })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.r).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<EntityRecord> {
        let r = serde_yaml::from_str::<entity::EntityRecord>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(EntityRecord { r })
    }
}

#[pyproto]
impl PyObjectProtocol for EntityRecord {
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
