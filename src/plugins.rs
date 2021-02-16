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
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use uuid::Uuid;

use crate::FosZenohSession;
use crate::{to_pyerr, zrpc_to_pyerr};

#[pyclass]
#[derive(Clone)]
pub struct OsClient {
    pub os: Arc<rustOSclient>,
}

#[pymodule]
pub fn plugins(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<OsClient>()?;

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
