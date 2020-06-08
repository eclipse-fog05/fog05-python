# Copyright (c) 2014,2018 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors: Gabriele Baldoni, ADLINK Technology Inc. - API v2

import uuid
import random
import time
import json
from enum import Enum
from fog05_sdk.interfaces import Constants
from fog05_sdk.interfaces.FDU import FDU
from fog05_sdk.interfaces.InfraFDU import InfraFDU
from mvar import MVar
import requests



# class RunningFDU(object):
#     def __init__(self, base_url, executor, instanceid, env,  sysid=Constants.default_system_id,
#                                     tenantid=Constants.default_tenant_id):
#         '''
#         Class: RunningFDU

#         This class implements the behaviour for a running FDU

#         attributes
#         ----------
#         instanceid : string
#             The associated FDU Instance ID

#         '''
#         self.sysid =  sysid
#         self.tenantid = tenantid
#         self.base_url = base_url
#         self.executor = executor
#         self.instanceid = instanceid
#         self.env = env
#         self.started = False
#         self.exit_code = None
#         self.log = None
#         self.var = MVar()
#         self.err_var = None

#     def run_job(self):
#         '''
#         Job that waits the FDU run to end
#         '''
#         self.stated = True
#         res = self.base_url.glob.actual.run_fdu_in_node(self.sysid, self.tenantid, self.instanceid, self.env)
#         if res.get('error') is not None:
#                 self.err_var = res.get('error')
#                 self.var.put(-255)
#         else:
#             self.err_var = None
#             self.var.put(int(res.get('result')))


#     def run(self):
#         '''
#         Submits the run_job
#         '''
#         if self.started is False:
#             self.exit_code = None
#             self.log =  None
#             self.executor.submit(self.run_job)
#         else:
#             raise ValueError('FDU is still running, wait it to finish before restaring')

#     def get_result(self):
#         '''
#         If the FDU is still running blocks until in ends, then returns its exit code and log

#         ---
#         returns
#         (int, string)
#         '''
#         if self.exit_code == None and self.log == None:
#             exit_code = self.var.get()
#             if self.err_var is not None:
#                 raise ValueError(self.err_var)
#             self.err_var = None
#             log = self.base_url.glob.actual.log_fdu_in_node(self.sysid, self.tenantid, self.instanceid)

#             if log.get('error') is not None:
#                 self.exit_code = int(exit_code)
#                 raise ValueError(log.get('error'))
#             self.started = False
#             self.exit_code = int(exit_code)
#             self.log = log.get('result')
#         return (self.exit_code, self.log)

#     def get_log(self):
#         '''
#         Returns the log of the FDU if it has ended
#         ---
#         returns
#         string
#         '''
#         return self.log

#     def get_code(self):
#         '''
#         Returns the exit code of the FDU if it has ended
#         ---
#         returns
#         int
#         '''
#         return self.exit_code



class FIMAPIRest(object):
    '''
    Class: FIMAPI

    This class implements the API to interact with Eclipse fog05 FIM

    attributes
    ----------
    descriptor : Descriptor
        Gives access to the descriptor API
    node : Node
        Gives access to the node API
    plugin : Plugin
        Gives access to the plugin API
    network : Network
        Gives access to the descriptor API
    fdu : FDUAPI
        Gives access to the FDU API
    image : Image
        Gives access to the image API
    flavor : Flavor
        Gives access to the flavor API

    '''

    def __init__(self, locator='http://127.0.0.1:5000',
                 sysid=Constants.default_system_id,
                 tenantid=Constants.default_tenant_id):

        self.base_url = locator
        self.sysid = sysid
        self.tenantid = tenantid
        self.node = self.Node(self.base_url, self.sysid, self.tenantid)
        self.plugin = self.Plugin(self.base_url, self.sysid, self.tenantid)
        self.network = self.Network(self.base_url, self.sysid, self.tenantid)
        self.fdu  = self.FDUAPI(self.base_url, self.sysid, self.tenantid, self.executor)
        self.image = self.Image(self.base_url, self.sysid, self.tenantid)
        self.flavor = self.Flavor(self.base_url, self.sysid, self.tenantid)


    def close(self):
        '''
        Closes the FIMAPI
        '''
        pass

    class Node(object):
        '''
        Class: Node
        This class encapsulates API for Nodes
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id):

            if base_url is None:
                raise RuntimeError('Yaks base_url cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid

        def list(self):
            '''
            Gets all nodes in the current system/tenant

            returns
            -------
            string list

            '''
            url = '{}/nodes/list'.format(self.base_url)
            return json.loads(str(requests.get(url).content))

        def info(self, node_uuid):
            '''
            Provides all information about the given node

            parameters
            ----------
            node_uuid : string
                UUID of the node

            returns
            -------
            dictionary
            '''
            url = '{}/nodes/{}/info'.format(self.base_url, node_uuid)
            return json.loads(str(requests.get(url).content))

        def status(self, node_uuid):
            '''
            Provides all status information about the given node,

            parameters
            ----------
            node_uuid : string
                UUID of the node

            returns
            -------
            dictionary
            '''
            url = '{}/nodes/{}/status'.format(self.base_url, node_uuid)
            return json.loads(str(requests.get(url).content))

        def plugins(self, node_uuid):
            '''
            Gets the list of plugins in the given node

            parameters
            ----------
            node_uuid : string
                UUID of the node

            returns
            -------
            string list
            '''
            url = '{}/nodes/{}/plugins/list'.format(self.base_url, node_uuid)
            return json.loads(str(requests.get(url).content))

    class Plugin(object):
        '''
        Class: Plugin
        This class encapsulates API for Plugins
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id):

            if base_url is None:
                raise RuntimeError('Yaks base_url cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid

        def info(self, plugin_uuid, node_uuid):
            '''
            Gets information about the given plugin in the given node

            parameters
            ----------
            plugin_uuid : string
                UUID of the plugin
            node_uuid : string
                UUID of the node

            returns
            -------
            dictionary
            '''
            url = '{}/nodes/{}/plugins/{}/info'.format(self.base_url, node_uuid, plugin_uuid)
            return json.loads(str(requests.get(url).content))

    class Network(object):
        '''
        Class: Plugin
        This class encapsulates API for networks
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id):

            if base_url is None:
                raise RuntimeError('Yaks base_url cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid

        def add_network(self, descriptor):
            '''
            Registers a network in the system catalog

            Needs at least one node in the system!

            parameters
            ----------
            descriptor : dictionary
                network descriptor

            returns
            -------
            string
            '''

            descriptor.update({'status': 'add'})
            net_id = descriptor.get('uuid')
            if net_id is None:
                net_id = uuid.uuid4()
                descriptor['uuid'] = net_id

            url = '{}/networks/{}/info'.format(self.base_url, net_id)
            res = json.loads(str(requests.put(url, descriptor).content))

            return net_id

        def remove_network(self, net_uuid):
            '''
            Removes the given network from the system catalog
            Needs at least one node in the system!

            parameters
            ----------
            net_uuid : string
                UUID of network

            returns
            -------
            bool
            '''
            url = '{}/networks/{}/info'.format(self.base_url, net_uuid)
            descriptor = json.loads(str(requests.get(url).content))
            if descriptor is None:
                return None
            descriptor.update({'status': 'remove'})
            res = json.loads(str(requests.put(url, descriptor).content))
            return net_uuid

        def add_network_to_node(self, descriptor, nodeid):
            '''
            Creates the given virtual network in the given node

            parameters
            ----------
            descriptor : dictionary
                network descriptor
            nodeid : string
                UUID of node

            returns
            -------
            dictionary
            '''
            net_id = descriptor.get('uuid')
            url = '{}/networks/{}/info'.format(self.base_url, net_id)
            net = json.loads(str(requests.get(url).content))

            if net is not None:
                return net
            url = '{}/nodes/{}/networks/{}/info'.format(self.base_url, nodeid, net_id)
            res = json.loads(str(requests.put(url, descriptor).content))
            if res.get('error') is not None:
                raise FIMTaskFailedException('Got  Error {} with message {}'.format(res['error'], res['error_msg']))
            return res['result']

        def remove_network_from_node(self, netid, nodeid):
            '''
            Removes the given virtual network from the given node

            parameters
            ----------
            netid : string
                network uuid
            nodeid : string
                UUID of node

            returns
            -------
            dictionary
            '''
            url = '{}/nodes/{}/networks/{}/info'.format(self.base_url, nodeid, netid)
            res = json.loads(str(requests.delete(url).content))
            if res.get('error') is not None:
                raise FIMTaskFailedException('Got  Error {} with message {}'.format(res['error'], res['error_msg']))
            return res['result']


        # def add_connection_point(self, cp_descriptor):
        #     '''
        #     Registers a connection point in the system catalog

        #     Needs at least one node in the system!

        #     parameters
        #     ----------
        #     descriptor : dictionary
        #         connection descriptor

        #     returns
        #     -------
        #     bool
        #     '''
        #     cp_descriptor.update({'status': 'add'})
        #     cp_id = cp_descriptor.get('uuid')
        #     self.base_url.glob.desired.add_network_port(
        #         self.sysid, self.tenantid, cp_id, cp_descriptor)

        # def delete_connection_point(self, cp_uuid):
        #     '''
        #     Removes the given connection point from the system catalog
        #     Needs at least one node in the system!

        #     parameters
        #     ----------
        #     cp_uuid : string
        #         UUID of connection point

        #     returns
        #     -------
        #     bool
        #     '''
        #     descriptor = self.base_url.glob.actual.get_network_port(
        #         self.sysid, self.tenantid, cp_uuid)
        #     descriptor.update({'status': 'remove'})
        #     self.base_url.glob.desired.add_network_port(
        #         self.sysid, self.tenantid, cp_uuid, descriptor)

        # def connect_cp_to_network(self, cp_uuid, net_uuid):
        #     '''
        #     Connects the given connection point to the given network

        #     parameters
        #     ----------
        #     cp_uuid : string
        #         UUID of the connection point
        #     net_uuid : string
        #         UUID of the virtual network

        #     returns
        #     -------
        #     string
        #     '''
        #     ports = self.base_url.glob.actual.get_all_nodes_network_ports(self.sysid, self.tenantid)
        #     node = None
        #     port_info = None
        #     for p in ports:
        #         n, pid = p
        #         if pid == cp_uuid:
        #             port_info = self.base_url.glob.actual.get_node_network_port(self. sysid, self.tenantid, n, pid)
        #             node = n
        #     if node is None or port_info is None:
        #         raise ValueError('Connection point {} not found'.format(cp_uuid))
        #     res = self.base_url.glob.actual.add_node_port_to_network(self.sysid, self.tenantid, node, port_info['uuid'], net_uuid)
        #     if res.get('result') is not None:
        #         return cp_uuid
        #     raise ValueError('Error connecting: {} with message {}'.format(res['error'], res['error_msg']))

        # def disconnect_cp(self, cp_uuid):
        #     '''
        #     Disconnects the given connection point

        #     parameters
        #     ----------
        #     cp_uuid : string
        #         UUID of connection point

        #     returns
        #     -------
        #     string
        #     '''
        #     ports = self.base_url.glob.actual.get_all_nodes_network_ports(self.sysid, self.tenantid)
        #     node = None
        #     port_info = None
        #     for p in ports:
        #         n, pid = p
        #         if pid == cp_uuid:
        #             port_info = self.base_url.glob.actual.get_node_network_port(self. sysid, self.tenantid, n, pid)
        #             node = n
        #     if node is None or port_info is None:
        #         raise ValueError('Connection point {} not found'.format(cp_uuid))
        #     res = self.base_url.glob.actual.remove_node_port_from_network(self.sysid, self.tenantid, node, port_info['uuid'])
        #     if res.get('result') is not None:
        #         return cp_uuid
        #     raise ValueError('Error connecting: {} with message {}'.format(res['error'], res['error_msg']))

        # def add_router(self, nodeid, descriptor):
        #     '''
        #     Creates the given virtual router in the given node

        #     parameters
        #     ----------
        #     descriptor : dictionary
        #         descriptor of the router
        #     nodeid : string
        #         UUID of the node

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     router_id = descriptor.get('uuid')
        #     self.base_url.glob.desired.add_node_network_router(
        #         self.sysid, self.tenantid, nodeid, router_id, descriptor)
        #     router_info = self.base_url.glob.actual.get_node_network_router(
        #         self.sysid, self.tenantid, nodeid, router_id)
        #     while router_info is None:
        #             router_info = self.base_url.glob.actual.get_node_network_router(
        #         self.sysid, self.tenantid, nodeid, router_id)
        #     return router_info


        # def remove_router(self, node_id, router_id):
        #     '''
        #     Removes the given virtual router in the given node

        #     parameters
        #     ----------
        #     router_id : string
        #         UUID of the router
        #     node_id : string
        #         UUID of the node

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     self.base_url.glob.desired.remove_node_network_router(
        #         self.sysid, self.tenantid, node_id, router_id)

        # def add_router_port(self, nodeid, router_id, port_type, vnet_id=None, ip_address=None):
        #     '''
        #     Adds a port to the given virtual router

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node
        #     router_id : string
        #         UUID of the virtual router
        #     port_type : string
        #         kind of the port to be added (INTERNAL, EXTERNAL)
        #     vnet_id : string
        #         eventual network to be connected
        #     ip_address : string
        #         eventual address for the new router port

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     if port_type.upper() not in ['EXTERNAL', 'INTERNAL']:
        #         raise ValueError('port_type can be only one of : INTERNAL, EXTERNAL')

        #     port_type = port_type.upper()
        #     return self.base_url.glob.actual.add_port_to_router(self.sysid, self.tenantid, nodeid, router_id, port_type, vnet_id, ip_address)

        # def remove_router_port(self, nodeid, router_id, vnet_id):
        #     '''
        #     Removes a port from the given router

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node
        #     router_id : string
        #         UUID of the virtual router
        #     vnet_id : string
        #         network to be disconnected

        #     returns
        #     -------
        #     dictionary

        #     '''
        #     return self.base_url.glob.actual.remove_port_from_router(self.sysid, self.tenantid, nodeid, router_id, vnet_id)



        # def create_floating_ip(self, nodeid):
        #     '''
        #     Creates a floating IP in the given node

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     return self.base_url.glob.actual.add_node_floatingip(self.sysid, self.tenantid, nodeid)

        # def delete_floating_ip(self, nodeid, ip_id):
        #     '''
        #     Deletes the given floating IP from the given node

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node
        #     ip_id : string
        #         UUID of the floating IP

        #     returns
        #     -------
        #     dictionary

        #     '''
        #     return self.base_url.glob.actual.remove_node_floatingip(self.sysid, self.tenantid, nodeid, ip_id)


        # def assign_floating_ip(self, nodeid, ip_id, cp_id):
        #     '''
        #     Assigns the given floating IP to the given conncetion point in the given node

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node
        #     ip_id : string
        #         UUID of the floating IP
        #     cp_id : string
        #         UUID of the connection point

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     return self.base_url.glob.actual.assign_node_floating_ip(self.sysid, self.tenantid, nodeid, ip_id, cp_id)

        # def retain_floating_ip(self, nodeid, ip_id, cp_id):
        #     '''
        #     Retains the given floating IP from the given connection point in the given node

        #     parameters
        #     ----------
        #     nodeid : string
        #         UUID of the node
        #     ip_id : string
        #         UUID of the floating IP
        #     cp_id : string
        #         UUID of the connection point

        #     returns
        #     -------
        #     dictionary
        #     '''
        #     return self.base_url.glob.actual.retain_node_floating_ip(self.sysid, self.tenantid, nodeid, ip_id, cp_id)

        def list(self):
            '''
            Gets all networks registered in the system catalog

            returns
            -------
            string list
            '''
            url = '{}/networks/list'.format(self.base_url)
            return json.loads(str(requests.get(url).content))


    class FDUAPI(object):
        '''
        Class: FDUAPI
        This class encapsulates API for FDUs
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id, executor=None):

            if base_url is None or executor is None:
                raise RuntimeError('Yaks base_url or executor cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid
            self.executor = executor

        def __wait_node_fdu_state_change(self, instanceid, state):
            '''
            Waits an FDU instance state to change

            parameters
            ----------
            instanceid : string
                UUID of instance
            state : string
                new state

            returns
            --------
            dictionary

            '''

            url = '{}/fdu/instances/{}/info'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.get(url).content))

            while fdu_info is None:
                    fdu_info = json.loads(str(requests.get(url).content))
            fdu = InfraFDU(fdu_info)
            es = fdu.get_status()
            while es.upper() not in [state, 'ERROR']:
                fdu_info = json.loads(str(requests.get(url).content))
                fdu = InfraFDU(fdu_info)
                es = fdu.get_status()

            if es.upper() == 'ERROR':
                raise FIMTaskFailedException('Unable to change state to {} for FDU Instance: {} Errno: {} Msg: {}'.format(
                    state, instanceid,fdu_info.get('error_code'), fdu_info.get('error_msg')))
            return fdu_info

        def __wait_specific_node_fdu_state_change(self, nodeid, instanceid, state):
            '''
            Waits an FDU instance state to change

            parameters
            ----------
            instanceid : string
                UUID of instance
            state : string
                new state

            returns
            --------
            dictionary

            '''
            url = '{}/nodes/{}/fdu/instances/{}/info'.format(self.base_url, nodeid ,instanceid)

            fdu_info = json.loads(str(requests.get(url).content))
            while fdu_info is None:
                    fdu_info = json.loads(str(requests.get(url).content))
            fdu = InfraFDU(fdu_info)
            es = fdu.get_status()
            while es.upper() not in [state, 'ERROR']:
                fdu_info = json.loads(str(requests.get(url).content))
                fdu = InfraFDU(fdu_info)
                es = fdu.get_status()

            if es.upper() == 'ERROR':
                raise FIMTaskFailedException('Unable to change state to {} for FDU Instance: {} Errno: {} Msg: {}'.format(
                    state, instanceid,fdu_info.get('error_code'), fdu_info.get('error_msg')))
            return fdu_info

        def onboard(self, descriptor):
            '''
            Registers an FDU descriptor in the system catalog
            Needs at least one node in the system!

            parameters
            ----------
            descriptor : FDU
                FDU descriptor

            returns
            -------
            FDU
            '''

            if not isinstance(descriptor, FDU):
                raise FIMTaskFailedException('descriptor should be of type FDU; actual type: {}'.format(type(descriptor)))


            url = '{}/fdu/{}/info'.format(self.base_url, descriptor.get_uuid())

            res = json.loads(str(requests.put(url, descriptor.to_json()).content))
            if res.get('result') is None:
                raise FIMTaskFailedException('Error during onboarding {} with message {}'.format(res['error'], res['error_msg']))
            return FDU(res['result'])


        def offload(self, fdu_uuid):
            '''
            Removes the given FDU from the system catalog
            Needs at least one node in the system!

            parameters
            ----------
            fdu_uuid : string
                UUID of fdu

            returns
            --------
            string
            '''
            url = '{}/fdu/{}/info'.format(self.base_url, fdu_uuid)

            res = json.loads(str(requests.delete(url).content))
            return fdu_uuid


        def define(self, fduid, node_uuid=None, wait=True):
            '''
            Defines the given fdu in the given node

            Instance UUID is system-wide unique

            parameters
            ----------
            fduid : string
                UUID of the FDU
            node_uuid : string
                UUID of the node optional
            wait : bool
                optional, call will block until FDU is defined
            returns
            -------
            InfraFDU
            '''


            if node_uuid is not None:


                url = '{}/fdu/{}/info'.format(self.base_url, fduid)
                desc = json.loads(str(requests.get(url).content))
                if desc is None:
                    raise FIMNotFoundException('FDU with this UUID not found in the catalog')



                url = '{}/nodes/{}/fdu/{}/define'.format(self.base_url, node_uuid ,fduid)
                res = json.loads(str(requests.put(url).content))
                if res.get('error') is not None:
                    raise FIMTaskFailedException('Got  Error {} with message {}'.format(res['error'], res['error_msg']))
                if wait:
                    self.__wait_node_fdu_state_change(res['result']['uuid'],'DEFINE')
                return InfraFDU(res['result'])
            else:

                url = '{}/fdu/{}/schedule'.format(self.base_url, fduid)
                res = json.loads(str(requests.put(url).content))
                if res.get('error') is not None:
                    raise FIMTaskFailedException('Got  Error {} with message {}'.format(res['error'], res['error_msg']))
                if wait:
                    self.__wait_node_fdu_state_change(res['result']['uuid'],'DEFINE')
                return InfraFDU(res['result'])


        def undefine(self, instanceid):
            '''
            Undefines the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance


            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/info'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.delete(url).content))

            return instanceid

        def configure(self, instanceid, wait=True):
            '''
            Configures the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            wait : bool
                optional, call will block until FDU is configured

            returns
            -------
            string
            '''
            url = '{}/fdu/instances/{}/configure'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url).content))

            return instanceid

        def clean(self, instanceid, wait=True):
            '''
            Cleans the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            wait : bool
                optional, call will block until FDU is cleaned

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/clean'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url).content))

        def start(self, instanceid, env=""):
            '''
            Starts the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            env : string
                Environment variables to be configured at the starting of the FDU in the format MYVAR=MYVALUE,MYVAR2=MYVALUE2,...
            wait : bool
                optional, call will block until FDU is started

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/start'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url, env).content))

            return instanceid


        # def run(self, instanceid, env=""):
        #     '''
        #     Runs and waits the given instance unit it ends
        #     returns a RunningFDU object where wait the FDU to end its execution.

        #     The RunningFDU object can be used to re-run the given FDU

        #     parameters
        #     ----------
        #     instanceid : string
        #         UUID of instance
        #     env : string
        #         Environment variables to be configured at the starting of the FDU in the format MYVAR=MYVALUE,MYVAR2=MYVALUE2,...
        #     returns
        #     -------
        #     RunningFDU
        #     '''
        #     res = RunningFDU(self.base_url, self.executor, instanceid, env)
        #     res.run()
        #     return res

        def stop(self, instanceid, wait=True):
            '''
            Stops the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            wait : bool
                optional, call will block until FDU is stopeed

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/stop'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url).content))

            return instanceid


        def pause(self, instanceid, wait=True):
            '''
            Pauses the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            wait : bool
                optional, call will block until FDU is paused

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/pause'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url).content))

            return instanceid

        def resume(self, instanceid, wait=True):
            '''
            Resumes the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            wait : bool
                optional, call will block until FDU is resumed

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/resume'.format(self.base_url, instanceid)
            fdu_info = json.loads(str(requests.post(url).content))

            return instanceid

        def migrate(self, instanceid, destination_node_uuid, wait=True):
            '''
            Migrates the given instance

            parameters
            ----------
            instanceid : string
                UUID of instance
            destination_node_uuid : string
                UUID of destination node
            wait : bool
                optional, call will block until FDU is migrated

            returns
            -------
            string
            '''

            url = '{}/fdu/instances/{}/migrate/{}'.format(self.base_url, instanceid, destination_node_uuid)
            fdu_info = json.loads(str(requests.post(url).content))

            return instanceid


        # def instantiate(self, fduid, nodeid, wait=True):
        #     '''
        #     Instantiates the given fdu in the given node

        #     This functions calls: define, configure, start

        #     Instance UUID is system-wide unique

        #     parameters
        #     ----------
        #     fduid : string
        #         UUID of the FDU
        #     node_uuid : string
        #         UUID of the node
        #     wait : bool
        #         optional, call will block until FDU is defined

        #     returns
        #     -------
        #     InfraFDU
        #     '''
        #     instance_info = self.define(fduid, nodeid)
        #     time.sleep(0.5)
        #     instance_id = instance_info.get_uuid()
        #     self.configure(instance_id)
        #     time.sleep(0.5)
        #     self.start(instance_id)
        #     return instance_info

        # def terminate(self, instanceid, wait=True):
        #     '''
        #     Terminates the given instance

        #     This function calls: stop, clean, undefine

        #     parameters
        #     ----------
        #     instanceid : string
        #         UUID of instance


        #     returns
        #     -------
        #     string
        #     '''

        #     self.stop(instanceid)
        #     self.clean(instanceid)
        #     return self.undefine(instanceid)

        # def log(self, instanceid):
        #     '''
        #     Gets the log for the given FDU instance
        #     parameters
        #     ----------
        #     instanceid : string
        #         UUID of instance

        #     returns
        #     -------
        #     string
        #     '''
        #     url = '{}/fdu/instances/{}/log'.format(self.base_url, instanceid)
        #     fdu_info = json.loads(str(requests.get(url).content))

        #     return fdu_info

        # def ls(self, instanceid):
        #     '''
        #     Lists the file in the given FDU instance directory
        #     parameters
        #     ----------
        #     instanceid : string
        #         UUID of instance

        #     returns
        #     -------
        #     string list
        #     '''
        #     ls = self.base_url.glob.actual.ls_fdu_in_node(self.sysid, self.tenantid, instanceid)

        #     if ls.get('error') is not None:
        #         raise ValueError(ls.get('error'))
        #     return json.loads(ls.get('result'))

        # def get_file(self, instanceid, filename):
        #     '''
        #     Gets the given filename for the given FDU instance
        #     parameters
        #     ----------
        #     instanceid : string
        #         UUID of instance

        #     returns
        #     -------
        #     string
        #     '''
        #     data = self.base_url.glob.actual.file_fdu_in_node(self.sysid, self.tenantid, instanceid, filename)

        #     if data.get('error') is not None:
        #         raise ValueError(data.get('error'))
        #     return data.get('result')

        # def search(self, search_dict, node_uuid=None):
        #     '''
        #     Searches for flavors that satisfies the parameter

        #     parameters
        #     ----------
        #     search_dict : dictionary
        #         search parameters
        #     node_uuid : string
        #         optional node UUID where search

        #     returns
        #     -------
        #     string list
        #     '''
        #     raise NotImplementedError("Not yet...")

        def info(self, fdu_uuid):
            '''
            Gets information about the given FDU from the catalog

            parameters
            ----------
            fdu_uuid : string
                UUID of the FDU

            returns
            -------
            FDU
            '''

            url = '{}/fdu/{}/info'.format(self.base_url, fdu_uuid)
            data = json.loads(str(requests.get(url).content))

            fdu = FDU(data)
            return fdu


        def instance_info(self, instanceid):
            '''
            Gets information about the given instance

            parameters
            ----------
            instanceid : string
                UUID of the instance

            returns
            -------
            InfraFDU
            '''

            url = '{}/fdu/instances/{}/info'.format(self.base_url, instanceid)
            data = json.loads(str(requests.get(url).content))
            fdu = InfraFDU(data)
            return fdu

        def get_nodes(self, fdu_uuid):
            '''
            Gets all the node in which the given FDU is running

            parameters
            ----------
            fdu_uuid : string
                UUID of the FDU

            returns
            -------
            string list

            '''
            url = '{}/fdu/{}/nodes/list'.format(self.base_url, fdu_uuid)
            data = json.loads(str(requests.get(url).content))
            return data

        def list_node(self, node_uuid):
            '''
            Gets all the FDUs running in the given node

            parameters
            ---------
            node_uuid : string
                UUID of the node

            returns
            -------
            string list
            '''
            url = '{}/nodes/{}/fdu/list'.format(self.base_url, node_uuid)
            data = json.loads(str(requests.get(url).content))
            return data


        def instance_list(self, fduid):
            '''
            Gets all the instances of a given FDU

            parameters
            ----------
            fduid : string
                UUID of the FDU

            returns
            -------
            dictionary
                {node_id: [instances list]}
            '''

            url = '{}/fdu/{}/instances/list'.format(self.base_url, fduid)
            data = json.loads(str(requests.get(url).content))


        def list(self):
            '''
            Gets all the FDUs registered in the catalog

            returns
            -------
            string list
            '''
            url = '{}/fdu/list'.format(self.base_url)
            data = json.loads(str(requests.get(url).content))



    class Image(object):
        '''
        Class: Image
        This class encapsulates API for Images
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id):

            if base_url is None:
                raise RuntimeError('Yaks base_url cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid


        def add(self, descriptor):
            '''
            Registers an image in the system catalog
            Needs at least one not in the system

            parameters
            ----------
            descriptor : dictionary
                image descriptor

            returns
            -------
            string
            '''

            img_id = descriptor.get('uuid')
            if img_id is None:
                img_id = uuid.uuid4()
                descriptor['uuid'] = img_id

            url = '{}/images/{}/info'.format(self.base_url, img_id)
            data = json.loads(str(requests.put(url, descriptor).content))
            return img_id

        def get(self, image_uuid):
            '''
            Gets the information about the given image

            parameters
            ----------
            image_uuid : string
                UUID of image

            returns
            -------
            dictionary
            '''
            url = '{}/images/{}/info'.format(self.base_url, image_uuid)
            data = json.loads(str(requests.get(url).content))
            return data

        def remove(self, image_uuid):
            '''
            Removes the given image from the system catalog
            Needs at least one not in the system

            parameters
            ----------
            image_uuid : string

            returns
            -------
            string
            '''

            url = '{}/images/{}/info'.format(self.base_url, image_uuid)
            data = json.loads(str(requests.delete(url).content))
            return image_uuid

        def list(self):
            '''
            Gets all the images registered in the system catalog

            returns
            -------
            string list
            '''

            url = '{}/images/list'.format(self.base_url)
            data = json.loads(str(requests.get(url).content))
            return data

    class Flavor(object):
        '''
        Class: Flavor
        This class encapsulates API for Flavors
        '''

        def __init__(self, base_url=None, sysid=Constants.default_system_id,
                     tenantid=Constants.default_tenant_id):

            if base_url is None:
                raise RuntimeError('Yaks base_url cannot be none in API!')
            self.base_url = base_url
            self.sysid = sysid
            self.tenantid = tenantid

        def add(self, descriptor):
            '''
            Registers a flavor in the system catalog
            Needs at least one node in the system

            parameters
            ----------
            descriptor : dictionary
                flavor descriptor

            returns
            -------
            string
            '''
            flv_id = descriptor.get('uuid')
            if flv_id is None:
                flv_id = uuid.uuid4()
                descriptor['uuid'] = flv_id

            url = '{}/flavors/{}/info'.format(self.base_url, flv_id)
            data = json.loads(str(requests.put(url, descriptor).content))
            return flv_id

        def get(self, flavor_uuid):
            '''
            Gets information about the given flavor

            parameters
            ----------
            flavor_uuid : string
                UUID of flavor

            returns
            -------
            dictionary

            '''

            url = '{}/flavors/{}/info'.format(self.base_url, flavor_uuid)
            data = json.loads(str(requests.get(url).content))
            return data

        def remove(self, flavor_uuid):
            '''
            Removes the given flavor from the system catalog
            Needs at least one node in the system

            parameters
            ----------

            flavor_uuid : string
                UUID of flavor

            returns
            -------
            string
            '''

            url = '{}/flavors/{}/info'.format(self.base_url, flavor_uuid)
            data = json.loads(str(requests.delete(url).content))
            return flavor_uuid

        def list(self):
            '''
            Gets all the flavors registered in the system catalog

            returns
            -------
            string list
            '''

            url = '{}/flavors/list'.format(self.base_url)
            data = json.loads(str(requests.get(url).content))
            return data


class FIMAuthExcetpion(Exception):
    def __init__(self, message):
        super(FIMAuthExcetpion, self).__init__(message)


class FIMAResouceExistingException(Exception):
    def __init__(self, message):
        super(FIMAResouceExistingException, self).__init__(message)


class FIMNotFoundException(Exception):
    def __init__(self, message):
        super(FIMNotFoundException, self).__init__(message)


class FIMTaskFailedException(Exception):
    def __init__(self, message):
        super(FIMTaskFailedException, self).__init__(message)