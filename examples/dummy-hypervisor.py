###############################################################################
# Copyright (c) 2018,2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache Software License 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
# Contributors:
#   ADLINK fog05 team, <fog05@adlink-labs.tech>
###############################################################################

from fog05.im.fdu import FduRecord
from fog05 import FosZenohSession
from fog05.plugins import WrappedHypervisor, AgentClient
import sys
from pprint import pprint
import time


## This example is the Python version of:
## https://github.com/eclipse-fog05/fog05/blob/master/fog05-sdk/examples/dummy-hypervisor.rs
## It shows a basic example for an Hypervisor Plugin

def press_to_continue():
    input("Press enter to continue")


class DummyHypervisor(object):
    def __init__(self, agent):
        self.name = "DummyPython"
        self.agent = agent

    def define_fdu(self, descriptor):
        return FduRecord()

    def undefine_fdu(self, instance_uuid):
        return instance_uuid

    def clean_fdu(self, instance_uuid):
        return instance_uuid

    def start_fdu(self, instance_uuid):
        return instance_uuid

    def stop_fdu(self, instance_uuid):
        return instance_uuid

    def migrate_fdu(self, instance_uuid, destination_uuid):
        return instance_uuid

    def get_fdu_status(self, instance_uuid):
         return FduRecord()

    def run_fdu(self, instance_uuid):
        return instance_uuid

    def log_fdu(self, instance_uuid):
        return ""

    def ls_fdu(self, instance_uuid):
        return [""]

    def file_fdu(self, instance_uuid, filename):
        return ""


def main():

    locator = 'tcp/127.0.0.1:61189'
    zenoh_session  = FosZenohSession(locator)

    local_servers = AgentClient.find_servers(zenoh_session)
    if len(local_servers) > 0:

        agent = AgentClient(zenoh_session, local_servers[0])

        py_hypervisor = DummyHypervisor(agent)

        hypervisor = WrappedHypervisor(py_hypervisor, zenoh_session)
        hypervisor.start()
        time.sleep(2)

        py_hypervisor.agent.register_hv_plugin(hypervisor.instance_uuid(), py_hypervisor.name)

        press_to_continue()

        py_hypervisor.agent.unregister_plugin(hypervisor.instance_uuid())
        hypervisor.stop()
    else:
        print("No Agent found")

    print("Bye!")


if __name__=='__main__':
    main()