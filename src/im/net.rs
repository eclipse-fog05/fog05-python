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

use fog05_sdk::types;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use crate::{json_to_pyerr, utf8_to_pyerr, yaml_to_pyerr};

#[pyclass]
#[derive(Clone, Debug)]
pub struct VethKind {
    pub t: types::VETHKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VlanKind {
    pub t: types::VLANKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VxlanKind {
    pub t: types::VXLANKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct BridgeKind {
    pub t: types::BridgeKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct GreKind {
    pub t: types::GREKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct MacVlanKind {
    pub t: types::MACVLANKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualInterfaceKind {
    pub t: types::VirtualInterfaceKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualInterface {
    pub t: types::VirtualInterface,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VlanConfKind {
    pub t: types::VLANConfKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VxlanConfKind {
    pub t: types::VXLANConfKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualInterfaceConfigKind {
    pub t: types::VirtualInterfaceConfigKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualInterfaceConfig {
    pub t: types::VirtualInterfaceConfig,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct NetworkNamespace {
    pub t: types::NetworkNamespace,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct ConnectionPoint {
    pub t: types::ConnectionPoint,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct ConnectionPointConfig {
    pub t: types::ConnectionPointConfig,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct InterfaceKind {
    pub t: types::InterfaceKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Interface {
    pub t: types::Interface,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct IpVersion {
    pub t: types::IPVersion,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct MCastVxlanInfo {
    pub t: types::MCastVXLANInfo,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct TreeGreInfo {
    pub t: types::TreeGREInfo,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct P2pVxlanInfo {
    pub t: types::P2PVXLANInfo,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct P2pGreInfo {
    pub t: types::P2PGREInfo,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct LinkKind {
    pub t: types::LinkKind,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct IpConfiguration {
    pub t: types::IPConfiguration,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualNetwork {
    pub t: types::VirtualNetwork,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct VirtualNetworkConfig {
    pub t: types::VirtualNetworkConfig,
}

#[pymodule]
pub fn net(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VethKind>()?;
    m.add_class::<VlanKind>()?;
    m.add_class::<VxlanKind>()?;
    m.add_class::<BridgeKind>()?;
    m.add_class::<GreKind>()?;
    m.add_class::<MacVlanKind>()?;
    m.add_class::<VirtualInterfaceKind>()?;
    m.add_class::<VirtualInterface>()?;
    m.add_class::<VlanConfKind>()?;
    m.add_class::<VxlanConfKind>()?;
    m.add_class::<VirtualInterfaceConfig>()?;
    m.add_class::<NetworkNamespace>()?;
    m.add_class::<ConnectionPoint>()?;
    m.add_class::<ConnectionPointConfig>()?;
    m.add_class::<InterfaceKind>()?;
    m.add_class::<IpVersion>()?;
    m.add_class::<MCastVxlanInfo>()?;
    m.add_class::<TreeGreInfo>()?;
    m.add_class::<P2pVxlanInfo>()?;
    m.add_class::<P2pGreInfo>()?;
    m.add_class::<LinkKind>()?;
    m.add_class::<IpConfiguration>()?;
    m.add_class::<VirtualNetwork>()?;
    m.add_class::<VirtualInterfaceConfig>()?;

    Ok(())
}

#[pymethods]
impl VethKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VethKind> {
        let t = serde_json::from_str::<types::VETHKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VethKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VethKind> {
        let t = serde_yaml::from_str::<types::VETHKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VethKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VethKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VlanKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VlanKind> {
        let t = serde_json::from_str::<types::VLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VlanKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VlanKind> {
        let t = serde_yaml::from_str::<types::VLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VlanKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VlanKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VxlanKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VxlanKind> {
        let t = serde_json::from_str::<types::VXLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VxlanKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VxlanKind> {
        let t = serde_yaml::from_str::<types::VXLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VxlanKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VxlanKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl BridgeKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<BridgeKind> {
        let t = serde_json::from_str::<types::BridgeKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(BridgeKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<BridgeKind> {
        let t = serde_yaml::from_str::<types::BridgeKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(BridgeKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for BridgeKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl GreKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<GreKind> {
        let t = serde_json::from_str::<types::GREKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(GreKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<GreKind> {
        let t = serde_yaml::from_str::<types::GREKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(GreKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for GreKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl MacVlanKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<MacVlanKind> {
        let t = serde_json::from_str::<types::MACVLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(MacVlanKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<MacVlanKind> {
        let t = serde_yaml::from_str::<types::MACVLANKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(MacVlanKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for MacVlanKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualInterfaceKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualInterfaceKind> {
        let t = serde_json::from_str::<types::VirtualInterfaceKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualInterfaceKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualInterfaceKind> {
        let t = serde_yaml::from_str::<types::VirtualInterfaceKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualInterfaceKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualInterfaceKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualInterface {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualInterface> {
        let t = serde_json::from_str::<types::VirtualInterface>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualInterface { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualInterface> {
        let t = serde_yaml::from_str::<types::VirtualInterface>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualInterface { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualInterface {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VlanConfKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VlanConfKind> {
        let t = serde_json::from_str::<types::VLANConfKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VlanConfKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VlanConfKind> {
        let t = serde_yaml::from_str::<types::VLANConfKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VlanConfKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VlanConfKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VxlanConfKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VxlanConfKind> {
        let t = serde_json::from_str::<types::VXLANConfKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VxlanConfKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VxlanConfKind> {
        let t = serde_yaml::from_str::<types::VXLANConfKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VxlanConfKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VxlanConfKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualInterfaceConfigKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualInterfaceConfigKind> {
        let t = serde_json::from_str::<types::VirtualInterfaceConfigKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualInterfaceConfigKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualInterfaceConfigKind> {
        let t = serde_yaml::from_str::<types::VirtualInterfaceConfigKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualInterfaceConfigKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualInterfaceConfigKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualInterfaceConfig {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualInterfaceConfig> {
        let t = serde_json::from_str::<types::VirtualInterfaceConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualInterfaceConfig { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualInterfaceConfig> {
        let t = serde_yaml::from_str::<types::VirtualInterfaceConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualInterfaceConfig { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualInterfaceConfig {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl NetworkNamespace {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<NetworkNamespace> {
        let t = serde_json::from_str::<types::NetworkNamespace>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(NetworkNamespace { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<NetworkNamespace> {
        let t = serde_yaml::from_str::<types::NetworkNamespace>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(NetworkNamespace { t })
    }
}

#[pyproto]
impl PyObjectProtocol for NetworkNamespace {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl ConnectionPoint {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<ConnectionPoint> {
        let t = serde_json::from_str::<types::ConnectionPoint>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(ConnectionPoint { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<ConnectionPoint> {
        let t = serde_yaml::from_str::<types::ConnectionPoint>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(ConnectionPoint { t })
    }
}

#[pyproto]
impl PyObjectProtocol for ConnectionPoint {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl ConnectionPointConfig {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<ConnectionPointConfig> {
        let t = serde_json::from_str::<types::ConnectionPointConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(ConnectionPointConfig { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<ConnectionPointConfig> {
        let t = serde_yaml::from_str::<types::ConnectionPointConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(ConnectionPointConfig { t })
    }
}

#[pyproto]
impl PyObjectProtocol for ConnectionPointConfig {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl InterfaceKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<InterfaceKind> {
        let t = serde_json::from_str::<types::InterfaceKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(InterfaceKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<InterfaceKind> {
        let t = serde_yaml::from_str::<types::InterfaceKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(InterfaceKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for InterfaceKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl Interface {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<Interface> {
        let t = serde_json::from_str::<types::Interface>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(Interface { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<Interface> {
        let t = serde_yaml::from_str::<types::Interface>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(Interface { t })
    }
}

#[pyproto]
impl PyObjectProtocol for Interface {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl IpVersion {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<IpVersion> {
        let t = serde_json::from_str::<types::IPVersion>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(IpVersion { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<IpVersion> {
        let t = serde_yaml::from_str::<types::IPVersion>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(IpVersion { t })
    }
}

#[pyproto]
impl PyObjectProtocol for IpVersion {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl MCastVxlanInfo {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<MCastVxlanInfo> {
        let t = serde_json::from_str::<types::MCastVXLANInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(MCastVxlanInfo { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<MCastVxlanInfo> {
        let t = serde_yaml::from_str::<types::MCastVXLANInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(MCastVxlanInfo { t })
    }
}

#[pyproto]
impl PyObjectProtocol for MCastVxlanInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl TreeGreInfo {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<TreeGreInfo> {
        let t = serde_json::from_str::<types::TreeGREInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(TreeGreInfo { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<TreeGreInfo> {
        let t = serde_yaml::from_str::<types::TreeGREInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(TreeGreInfo { t })
    }
}

#[pyproto]
impl PyObjectProtocol for TreeGreInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl P2pVxlanInfo {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<P2pVxlanInfo> {
        let t = serde_json::from_str::<types::P2PVXLANInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(P2pVxlanInfo { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<P2pVxlanInfo> {
        let t = serde_yaml::from_str::<types::P2PVXLANInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(P2pVxlanInfo { t })
    }
}

#[pyproto]
impl PyObjectProtocol for P2pVxlanInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl P2pGreInfo {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<P2pGreInfo> {
        let t = serde_json::from_str::<types::P2PGREInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(P2pGreInfo { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<P2pGreInfo> {
        let t = serde_yaml::from_str::<types::P2PGREInfo>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(P2pGreInfo { t })
    }
}

#[pyproto]
impl PyObjectProtocol for P2pGreInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl LinkKind {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<LinkKind> {
        let t = serde_json::from_str::<types::LinkKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(LinkKind { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<LinkKind> {
        let t = serde_yaml::from_str::<types::LinkKind>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(LinkKind { t })
    }
}

#[pyproto]
impl PyObjectProtocol for LinkKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl IpConfiguration {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<IpConfiguration> {
        let t = serde_json::from_str::<types::IPConfiguration>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(IpConfiguration { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<IpConfiguration> {
        let t = serde_yaml::from_str::<types::IPConfiguration>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(IpConfiguration { t })
    }
}

#[pyproto]
impl PyObjectProtocol for IpConfiguration {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualNetwork {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualNetwork> {
        let t = serde_json::from_str::<types::VirtualNetwork>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualNetwork { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualNetwork> {
        let t = serde_yaml::from_str::<types::VirtualNetwork>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualNetwork { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualNetwork {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl VirtualNetworkConfig {
    #[cfg(feature = "json")]
    fn serialize_json(&self) -> PyResult<Vec<u8>> {
        Ok(serde_json::to_string(&self.t)
            .map_err(json_to_pyerr)?
            .into_bytes())
    }

    #[cfg(feature = "json")]
    #[staticmethod]
    fn deserialize_json(raw_data: &[u8]) -> PyResult<VirtualNetworkConfig> {
        let t = serde_json::from_str::<types::VirtualNetworkConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(json_to_pyerr)?;
        Ok(VirtualNetworkConfig { t })
    }

    #[cfg(feature = "yaml")]
    fn serialize_yaml(&self) -> PyResult<Vec<u8>> {
        serde_yaml::to_vec(&self.t).map_err(yaml_to_pyerr)
    }

    #[cfg(feature = "yaml")]
    #[staticmethod]
    fn deserialize_yaml(raw_data: &[u8]) -> PyResult<VirtualNetworkConfig> {
        let t = serde_yaml::from_str::<types::VirtualNetworkConfig>(
            std::str::from_utf8(raw_data).map_err(utf8_to_pyerr)?,
        )
        .map_err(yaml_to_pyerr)?;
        Ok(VirtualNetworkConfig { t })
    }
}

#[pyproto]
impl PyObjectProtocol for VirtualNetworkConfig {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}
