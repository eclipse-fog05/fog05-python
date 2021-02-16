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

use async_std::sync::Arc;
use async_std::task;
use fog05_sdk::agent::OSClient as rustOSclient;
use fog05_sdk::plugins::NetworkingPluginClient as rustNetworkingClient;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use uuid::Uuid;

use crate::im::net::{
    ConnectionPoint, NetworkNamespace, VirtualInterface, VirtualInterfaceConfig, VirtualNetwork,
};

use crate::FosZenohSession;
use crate::{to_pyerr, zrpc_to_pyerr};

#[pyclass]
#[derive(Clone)]
pub struct OsClient {
    pub os: Arc<rustOSclient>,
}

#[pyclass]
#[derive(Clone)]
pub struct NetClient {
    pub net: Arc<rustNetworkingClient>,
}

#[pymodule]
pub fn plugins(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<OsClient>()?;
    m.add_class::<NetClient>()?;

    Ok(())
}

// Cannot use async until: https://github.com/PyO3/pyo3/pull/1406

#[pymethods]
impl OsClient {
    #[new]
    fn new(zenoh: FosZenohSession, server_uuid: String) -> PyResult<Self> {
        let server_uuid = Uuid::parse_str(&server_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let os = Arc::new(rustOSclient::new(zenoh.zsession.clone(), server_uuid));
            Ok(Self { os })
        })
    }

    #[staticmethod]
    fn find_servers(zenoh: FosZenohSession) -> PyResult<Vec<String>> {
        task::block_on(async {
            let res = rustOSclient::find_servers(zenoh.zsession.clone())
                .await
                .map_err(zrpc_to_pyerr)?;
            let servers: Vec<String> = res.iter().map(|x| format!("{}", x)).collect();
            Ok(servers)
        })
    }

    #[staticmethod]
    fn find_local_servers(zenoh: FosZenohSession) -> PyResult<Vec<String>> {
        task::block_on(async {
            let res = rustOSclient::find_local_servers(zenoh.zsession.clone())
                .await
                .map_err(zrpc_to_pyerr)?;
            let servers: Vec<String> = res.iter().map(|x| format!("{}", x)).collect();
            Ok(servers)
        })
    }

    fn verify_server(&self) -> PyResult<bool> {
        task::block_on(async { self.os.verify_server().await.map_err(zrpc_to_pyerr) })
    }

    fn dir_exists(&self, dir_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .dir_exists(dir_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn create_dir(&self, dir_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .create_dir(dir_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn rm_dir(&self, dir_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .rm_dir(dir_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn download_file(&self, url: String, dest_path: String) -> PyResult<bool> {
        let url =
            url::Url::parse(&url).map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.os
                .download_file(url, dest_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn create_file(&self, file_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .create_file(file_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn rm_file(&self, file_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .rm_file(file_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn store_file(&self, content: Vec<u8>, file_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .store_file(content, file_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn read_file(&self, file_path: String) -> PyResult<Vec<u8>> {
        task::block_on(async {
            self.os
                .read_file(file_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn file_exists(&self, file_path: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .file_exists(file_path)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn execute_command(&self, cmd: String) -> PyResult<String> {
        task::block_on(async {
            self.os
                .execute_command(cmd)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn send_signal(&self, signal: u8, pid: u32) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .send_signal(signal, pid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn check_if_pid_exists(&self, pid: u32) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .check_if_pid_exists(pid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn get_interface_type(&self, _iface: String) -> PyResult<String> {
        // Export for types is needed
        unimplemented!()
    }

    fn set_interface_unavailable(&self, iface: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .set_interface_unavailable(iface)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn set_interface_available(&self, iface: String) -> PyResult<bool> {
        task::block_on(async {
            self.os
                .set_interface_available(iface)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }

    fn get_local_mgmt_address(&self, _iface: String) -> PyResult<String> {
        // Export for types is needed
        unimplemented!()
    }

    fn get_local_mgmt_interface(&self) -> PyResult<String> {
        task::block_on(async {
            self.os
                .get_local_mgmt_interface()
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)
        })
    }
}

#[pyproto]
impl PyObjectProtocol for OsClient {
    fn __str__(&self) -> PyResult<String> {
        Ok("OSClient".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl NetClient {
    #[new]
    fn new(zenoh: FosZenohSession, server_uuid: String) -> PyResult<Self> {
        let server_uuid = Uuid::parse_str(&server_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let net = Arc::new(rustNetworkingClient::new(
                zenoh.zsession.clone(),
                server_uuid,
            ));
            Ok(Self { net })
        })
    }

    #[staticmethod]
    fn find_servers(zenoh: FosZenohSession) -> PyResult<Vec<String>> {
        task::block_on(async {
            let res = rustNetworkingClient::find_servers(zenoh.zsession.clone())
                .await
                .map_err(zrpc_to_pyerr)?;
            let servers: Vec<String> = res.iter().map(|x| format!("{}", x)).collect();
            Ok(servers)
        })
    }

    #[staticmethod]
    fn find_local_servers(zenoh: FosZenohSession) -> PyResult<Vec<String>> {
        task::block_on(async {
            let res = rustNetworkingClient::find_local_servers(zenoh.zsession.clone())
                .await
                .map_err(zrpc_to_pyerr)?;
            let servers: Vec<String> = res.iter().map(|x| format!("{}", x)).collect();
            Ok(servers)
        })
    }

    fn verify_server(&self) -> PyResult<bool> {
        task::block_on(async { self.net.verify_server().await.map_err(zrpc_to_pyerr) })
    }

    fn create_default_virtual_network(&self, dhcp: bool) -> PyResult<VirtualNetwork> {
        task::block_on(async {
            let t = self
                .net
                .create_default_virtual_network(dhcp)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualNetwork { t })
        })
    }

    fn create_virtual_network(&self, vnet_uuid: String) -> PyResult<VirtualNetwork> {
        let vnet_uuid = Uuid::parse_str(&vnet_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .create_virtual_network(vnet_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualNetwork { t })
        })
    }

    fn get_virtual_network(&self, vnet_uuid: String) -> PyResult<VirtualNetwork> {
        let vnet_uuid = Uuid::parse_str(&vnet_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .get_virtual_network(vnet_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualNetwork { t })
        })
    }

    fn delete_virtual_network(&self, vnet_uuid: String) -> PyResult<VirtualNetwork> {
        let vnet_uuid = Uuid::parse_str(&vnet_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_virtual_network(vnet_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualNetwork { t })
        })
    }

    fn create_connection_point(&self) -> PyResult<ConnectionPoint> {
        task::block_on(async {
            let t = self
                .net
                .create_connection_point()
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(ConnectionPoint { t })
        })
    }

    fn get_connection_point(&self, cp_uuid: String) -> PyResult<ConnectionPoint> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .get_connection_point(cp_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(ConnectionPoint { t })
        })
    }

    fn delete_connection_point(&self, cp_uuid: String) -> PyResult<String> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_connection_point(cp_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(format!("{}", t))
        })
    }

    fn create_virtual_interface(&self, intf: VirtualInterfaceConfig) -> PyResult<VirtualInterface> {
        task::block_on(async {
            let t = self
                .net
                .create_virtual_interface(intf.t)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn get_virtual_interface(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .get_virtual_interface(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn delete_virtual_interface(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_virtual_interface(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn create_virtual_bridge(&self, br_name: String) -> PyResult<VirtualInterface> {
        task::block_on(async {
            let t = self
                .net
                .create_virtual_bridge(br_name)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn get_virtual_bridge(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .get_virtual_bridge(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn delete_virtual_bridge(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_virtual_bridge(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn create_network_namespace(&self) -> PyResult<NetworkNamespace> {
        task::block_on(async {
            let t = self
                .net
                .create_network_namespace()
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(NetworkNamespace { t })
        })
    }

    fn get_network_namespace(&self, ns_uuid: String) -> PyResult<NetworkNamespace> {
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .get_network_namespace(ns_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(NetworkNamespace { t })
        })
    }

    fn delete_network_namespace(&self, ns_uuid: String) -> PyResult<NetworkNamespace> {
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_network_namespace(ns_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(NetworkNamespace { t })
        })
    }

    fn bind_interface_to_connection_point(
        &self,
        intf_uuid: String,
        cp_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .bind_interface_to_connection_point(intf_uuid, cp_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn unbind_interface_from_connection_point(
        &self,
        intf_uuid: String,
        cp_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .bind_interface_to_connection_point(intf_uuid, cp_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn bind_connection_point_to_virtual_network(
        &self,
        intf_uuid: String,
        vnet_uuid: String,
    ) -> PyResult<ConnectionPoint> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let vnet_uuid = Uuid::parse_str(&vnet_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .bind_connection_point_to_virtual_network(intf_uuid, vnet_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(ConnectionPoint { t })
        })
    }

    fn unbind_connection_point_from_virtual_network(
        &self,
        intf_uuid: String,
        vnet_uuid: String,
    ) -> PyResult<ConnectionPoint> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let vnet_uuid = Uuid::parse_str(&vnet_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .unbind_connection_point_from_virtual_network(intf_uuid, vnet_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(ConnectionPoint { t })
        })
    }

    fn get_interface_addresses(&self, intf_uuid: String) -> PyResult<Vec<String>> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;

        task::block_on(async {
            let t = self
                .net
                .get_interface_addresses(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            let addresses: Vec<String> = t.iter().map(|x| format!("{}", x)).collect();
            Ok(addresses)
        })
    }

    fn get_overlay_iface(&self) -> PyResult<String> {
        task::block_on(async {
            let t = self
                .net
                .get_overlay_iface()
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(t)
        })
    }

    fn get_vlan_face(&self) -> PyResult<String> {
        task::block_on(async {
            let t = self
                .net
                .get_vlan_face()
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(t)
        })
    }

    fn create_macvlan_interface(&self, master_intf: String) -> PyResult<VirtualInterface> {
        task::block_on(async {
            let t = self
                .net
                .create_macvlan_interface(master_intf)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn delete_macvan_interface(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_macvan_interface(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn move_interface_info_namespace(
        &self,
        intf_uuid: String,
        ns_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .move_interface_info_namespace(intf_uuid, ns_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn move_interface_into_default_namespace(
        &self,
        intf_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .move_interface_into_default_namespace(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn rename_virtual_interface(
        &self,
        intf_uuid: String,
        intf_name: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .rename_virtual_interface(intf_uuid, intf_name)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn attach_interface_to_bridge(
        &self,
        intf_uuid: String,
        br_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let br_uuid = Uuid::parse_str(&br_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .attach_interface_to_bridge(intf_uuid, br_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn detach_interface_from_bridge(&self, intf_uuid: String) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .detach_interface_from_bridge(intf_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn create_virtual_interface_in_namespace(
        &self,
        intf: VirtualInterfaceConfig,
        ns_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .create_virtual_interface_in_namespace(intf.t, ns_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn delete_virtual_interface_in_namespace(
        &self,
        intf_uuid: String,
        ns_uuid: String,
    ) -> PyResult<VirtualInterface> {
        let intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            let t = self
                .net
                .delete_virtual_interface_in_namespace(intf_uuid, ns_uuid)
                .await
                .map_err(zrpc_to_pyerr)?
                .map_err(to_pyerr)?;
            Ok(VirtualInterface { t })
        })
    }

    fn assing_address_to_interface(
        &self,
        intf_uuid: String,
        _address: Option<String>,
    ) -> PyResult<VirtualInterface> {
        let _intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        unimplemented!();
    }

    fn remove_address_from_interface(
        &self,
        intf_uuid: String,
        _address: String,
    ) -> PyResult<VirtualInterface> {
        let _intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        unimplemented!()
    }

    fn set_macaddres_of_interface(
        &self,
        intf_uuid: String,
        _address: String,
    ) -> PyResult<VirtualInterface> {
        let _intf_uuid = Uuid::parse_str(&intf_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        unimplemented!()
    }
}

#[pyproto]
impl PyObjectProtocol for NetClient {
    fn __str__(&self) -> PyResult<String> {
        Ok("NetClient".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}
