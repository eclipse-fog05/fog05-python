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
use fog05_sdk::api;
use fog05_sdk::zconnector::ZConnector;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use uuid::Uuid;
use zenoh::*;
use async_std::task;

use crate::{to_pyerr, zrpc_to_pyerr};

#[pyclass]
#[derive(Clone)]
pub struct FduApi {
    pub a: Arc<api::FDUApi>,
}

#[pymodule]
pub fn api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FduApi>()?;

    Ok(())
}


// Cannot use async until: https://github.com/PyO3/pyo3/pull/1406

#[pymethods]
impl FduApi {

    #[new]
    fn new(locator: String) -> PyResult<Self> {

        async fn _new(locator: String) -> FduApi {
            let zenoh = Arc::new(
                Zenoh::new(Properties::from(format!("mode=client;peer={}", locator)).into())
                    .await
                    .unwrap(),
            );
            let zsession = Arc::new(
                zenoh::net::open(Properties::from(format!("mode=client;peer={}", locator)).into())
                    .await
                    .unwrap(),
            );
            let zconnector = Arc::new(ZConnector::new(zenoh.clone(), None, None));
            let a = Arc::new(api::FDUApi::new(zconnector, zsession));
            FduApi { a }
        }
        task::block_on(async {
            Ok(_new(locator).await)
        })

    }

    fn onboard_fdu(&self, fdu : crate::im::fdu::FduDescriptor ) -> PyResult<String> {
        task::block_on(async {
            let fdu_uuid = self.a.onboard_fdu(fdu.d).await
            .map_err(to_pyerr)?;
            Ok(format!("{}",fdu_uuid))
        })
    }

    fn define_fdu(
        &self,
        fdu_uuid: String,
        node_uuid: Option<String>,
    ) -> PyResult<crate::im::fdu::FduRecord> {

        let fdu_uuid = Uuid::parse_str(&fdu_uuid).map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?;
        let node_uuid = match node_uuid {
            Some(id) => Some(Uuid::parse_str(&id).map_err(|err| PyErr::new::<crate::FError, _>(err.to_string()))?),
            None => None
        };

        task::block_on( async {

            let r =
            self.a
            .define_fdu(fdu_uuid, node_uuid)
            .await
            .map_err(to_pyerr)?;
            Ok(crate::im::fdu::FduRecord{r})
        })


    }
}

#[pyproto]
impl PyObjectProtocol for FduApi {
    fn __str__(&self) -> PyResult<String> {
        Ok("FduApi".to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}
