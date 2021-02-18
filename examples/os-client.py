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

from fog05.plugins import OsClient
from fog05 import FosZenohSession
import sys
from pprint import pprint


## This example is the Python version of:
## https://github.com/eclipse-fog05/fog05/blob/master/fog05-sdk/examples/agent-os.rs
## Thus shows the interaction between the OS Plugin from Python code

def main():

    locator = 'tcp/127.0.0.1:61189'
    zenoh_session  = FosZenohSession(locator)
    local_servers = OsClient.find_servers(zenoh_session)
    print(f'Servers {local_servers}')
    if len(local_servers) > 0:
        server_uuid = local_servers[0]
        client = OsClient(zenoh_session, server_uuid)
        print(f'Dir exists res: {client.dir_exists("/tmp")}')
        url = 'https://gist.githubusercontent.com/gabrik/24e664ff772837563acd59108bc724e5/raw/8eb18fdaef00a2bc2df3af8e4f50b3db514cfaa0/node-prepare.sh'
        print(f'Download res: {client.download_file(url,"/tmp/dest.sh")}')

    else:
        print('No Servers')
    print('Bye')


if __name__=='__main__':
    main()


    # {'uuid': None, 'id': 'test-ping-fdu', 'name': 'ping fdu', 'version': '0.3.0', 'fdu_version': '0.0.1', 'description': None, 'hypervisor': 'bare', 'image': None, 'hypervisor_specific': '{"cmd":"ping","args":["1.1.1.1"],"env":{}}', 'computation_requirements': {'cpu_arch': 'x86_64', 'cpu_min_freq': 0, 'cpu_min_count': 0, 'gpu_min_count': 0, 'fpga_min_count': 0, 'operating_system': None, 'ram_size_mb': 10, 'storage_size_mb': 1}, 'geographical_requirements': None, 'interfaces': [], 'storage': [], 'connection_points': [], 'configuration': None, 'migration_kind': 'COLD', 'replicas': None, 'depends_on': []}