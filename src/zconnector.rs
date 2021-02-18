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
use fog05_sdk::zconnector::{Global as rustGlobal, local::Local as rustLocal};
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use uuid::Uuid;

use crate::to_pyerr;
use crate::FosZenohSession;

#[pyclass]
#[derive(Clone)]
pub struct ZConnector {
    pub z: Arc<FosZenohSession>,
    pub local: Arc<Local>,
    pub global: Arc<Global>,
}

#[pyclass]
pub struct Local {
    pub t: rustLocal,
}

#[pyclass]
pub struct Global {
    pub t: rustGlobal,
}

#[pymodule]
pub fn zconnector(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ZConnector>()?;

    Ok(())
}

// Cannot use async until: https://github.com/PyO3/pyo3/pull/1406

#[pymethods]
impl ZConnector {
    #[new]
    pub fn new(z: FosZenohSession, sys_id: Option<String>, tenant_id: Option<String>) -> Self {
        let sys_id = match sys_id {
            Some(sys_id) => Some(Uuid::parse_str(&sys_id).unwrap()),
            None => None
        };
        let tenant_id = match tenant_id {
            Some(tenant_id) => Some(Uuid::parse_str(&tenant_id).unwrap()),
            None => None
        };
        Self {
            z : Arc::new(z.clone()),
            global: Arc::new(Global{t: rustGlobal::new(z.zenoh.clone(), sys_id, tenant_id)}),
            // We should get the node UUID from zenoh
            local: Arc::new(Local{t: rustLocal::new(z.zenoh, fog05_sdk::get_node_uuid().unwrap())}),
        }
    }
}

#[pymethods]
impl Global {

    fn get_system_info(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_system_info().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_system_config(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_system_config().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_nodes(&self) -> PyResult<Vec<String>> {
        task::block_on(async {
            self.t.get_all_nodes().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_node_info(&self, node_uuid: String) -> PyResult<String> {
        let node_uuid = Uuid::parse_str(&node_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_node_info(node_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_node_info(&self, node_uuid: String) -> PyResult<String> {
        let node_uuid = Uuid::parse_str(&node_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.remove_node_info(node_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_node_info(&self, node_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_node_info(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_node_status(&self, node_uuid: String) -> PyResult<String> {
        let node_uuid = Uuid::parse_str(&node_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_node_status(node_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_node_status(&self, node_uuid: String) -> PyResult<String> {
        let node_uuid = Uuid::parse_str(&node_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_node_status(node_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_node_status(&self, node_status: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_node_status(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_plugin(&self, node_uuid: String, plugin_uuid: String) -> PyResult<String> {
        let node_uuid = Uuid::parse_str(&node_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let plugin_uuid = Uuid::parse_str(&plugin_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_plugin(node_uuid, plugin_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_virtual_network(&self, net_uuid: String) -> PyResult<String> {
        let net_uuid = Uuid::parse_str(&net_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_virtual_network(net_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_virutal_network(&self, vnet_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_virutal_network(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_virtual_network(&self, net_uuid: String) -> PyResult<String> {
        let net_uuid = Uuid::parse_str(&net_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_virtual_network(net_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_connection_point(&self, cp_uuid: String) -> PyResult<String> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_connection_point(cp_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_connection_point(&self, cp_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_connection_point(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_connection_point(&self, cp_uuid: String) -> PyResult<String> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_connection_point(cp_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_interface(&self, iface_uuid: String) -> PyResult<String> {
        let iface_uuid = Uuid::parse_str(&iface_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_interface(iface_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_interface(&self, iface_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_interface(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_interface(&self, iface_uuid: String) -> PyResult<String> {
        let iface_uuid = Uuid::parse_str(&iface_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_interface(iface_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_fdu(&self, fdu_uuid: String) -> PyResult<String> {
        let fdu_uuid = Uuid::parse_str(&fdu_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_fdu(fdu_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_fdu(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_all_fdu().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_fdu(&self, fdu_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_fdu(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_fdu(&self, fdu_uuid: String) -> PyResult<String> {
        let fdu_uuid = Uuid::parse_str(&fdu_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.remove_fdu(fdu_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_instance(instance_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_fdu_instances(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_all_fdu_instances(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_instances(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_all_instances().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_instance(&self, add_instance: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_fdu(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_instance(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_entity(&self, entity_uuid: String) -> PyResult<String> {
        let entity_uuid = Uuid::parse_str(&entity_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_entity(entity_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_entity(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_all_entity().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_entity(&self, entity_uuid: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_fdu(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_entity(&self, entity_uuid: String) -> PyResult<String> {
        let entity_uuid = Uuid::parse_str(&entity_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.remove_entity(entity_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_entity_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_entity_instance(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_entity_instances(&self, entity_uuid: String) -> PyResult<String> {
        let entity_uuid = Uuid::parse_str(&entity_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_all_entity_instances(entity_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_entities_instances(&self) -> PyResult<String> {
        task::block_on(async {
            self.t
                .get_all_entities_instances()
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_entity_instance(&self, instance_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_fdu(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_entity_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_entity_instance(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Global {
    fn __str__(&self) -> PyResult<String> {
        Ok("Global".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

#[pymethods]
impl Local {

    fn get_node_info(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_node_info().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_node_info(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.remove_node_info().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_node_info(&self, node_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_node_info(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_node_status(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_node_status().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_node_status(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.remove_node_status().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_node_status(&self, node_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_node_status(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_plugin(&self, plugin_uuid: String) -> PyResult<String> {
        let plugin_uuid = Uuid::parse_str(&plugin_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_plugin(plugin_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_plugin(&self, plugin_uuid: String) -> PyResult<String> {
        let plugin_uuid = Uuid::parse_str(&plugin_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.remove_plugin(plugin_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_plugin(&self, plugin_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_plugin(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_virtual_network(&self, net_uuid: String) -> PyResult<String> {
        let net_uuid = Uuid::parse_str(&net_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_virtual_network(net_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_virtual_network(&self, net_uuid: String) -> PyResult<String> {
        let net_uuid = Uuid::parse_str(&net_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_virtual_network(net_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_virutal_network(&self, vnet_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_plugin(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn get_connection_point(&self, cp_uuid: String) -> PyResult<String> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_connection_point(cp_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_connection_point(&self, cp_uuid: String) -> PyResult<String> {
        let cp_uuid = Uuid::parse_str(&cp_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_connection_point(cp_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_connection_point(&self, cp_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_plugin(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // }
    }

    fn get_interface(&self, iface_uuid: String) -> PyResult<String> {
        let iface_uuid = Uuid::parse_str(&iface_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_interface(iface_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_interfaces(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_all_interfaces().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_interface(&self, iface_uuid: String) -> PyResult<String> {
        let iface_uuid = Uuid::parse_str(&iface_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_interface(iface_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_interface(&self, intf_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_plugin(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // }
    }

    fn get_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t.get_instance(instance_uuid).await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_fdu_instances(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_all_fdu_instances(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_all_instances(&self) -> PyResult<String> {
        task::block_on(async {
            self.t.get_all_instances().await.map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_instance(&self, instance_info: String) -> PyResult<String> {
        unimplemented!()
        // let node_uuid = Uuid::parse_str(&node_uuid)
        //     .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        // task::block_on(async {
        //     self.t.add_fdu(node_uuid).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // })
    }

    fn remove_instance(&self, instance_uuid: String) -> PyResult<String> {
        let instance_uuid = Uuid::parse_str(&instance_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_instance(instance_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn get_network_namespace(&self, ns_uuid: String) -> PyResult<String> {
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .get_network_namespace(ns_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn remove_network_namespace(&self, ns_uuid: String) -> PyResult<String> {
        let ns_uuid = Uuid::parse_str(&ns_uuid)
            .map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        task::block_on(async {
            self.t
                .remove_network_namespace(ns_uuid)
                .await
                .map_err(to_pyerr)?;
            unimplemented!()
        })
    }

    fn add_network_namespace(&self, ns_info: String) -> PyResult<String> {
        unimplemented!()
        // task::block_on(async {
        //     self.t.add_network_namespace(ns_info).await.map_err(to_pyerr)?;
        //     unimplemented!()
        // }
    }
}


#[pyproto]
impl PyObjectProtocol for Local {
    fn __str__(&self) -> PyResult<String> {
        Ok("Local".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}