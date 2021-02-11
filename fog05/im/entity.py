# Copyright (c) 2014,2021 Contributors to the Eclipse Foundation
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
# Contributors: Gabriele Baldoni, ADLINK Technology Inc.
# Python API and SDK


from enum import Enum
from fog05.im.fdu import FDUDescriptor
import cbor2
import uuid
import json
import yaml


class IPVersion(Enum):
    IPV4 = "IPV4"
    IPV6 = "IPV6"


class LinkKind(Enum):
    L2 = "L2"
    L3 = "L3"
    ELINE = "ELINE"
    ELAN = "ELAN"

class IPConfiguration(object):
    def __init__(self, **kwargs):
        self.subnet = None 
        self.gateway = None
        self.dhcp_range = None
        self.dns = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, tuple):
                        if key == "subnet" and len(value) == 2:
                            self.__dict__.update({key: value})
                        elif key == "dhcp_range" and len(value) == 2:
                            self.__dict__.update({key: value})
                    elif isinstance(value, list):
                        l = []
                        if key == "dns":
                            l = value
                        self.__dict__.update({key: l})

                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__


class VirtualNetwork(object):
    def __init__(self, **kwargs):
        self.uuid = None
        self.id = ""
        self.name = None
        self.is_mgmt = False
        self.link_kind = LinkKind.L2
        self.ip_version = IPVersion.IPV4
        self.ip_configuration = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        if key == "ip_configuration":
                            self.__dict__.update({key: IPConfiguration(**value)})
                    elif isinstance(value, list):
                        pass
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})
        
    def as_dict(self):
        d = self.__dict__
        d.update({"ip_configuration":d['ip_configuration'].as_dict()})
        return d

class EntityDescriptor(object):
    def __init__(self, **kwargs):
        self.uuid = None
        self.id = ""
        self.version = ""
        self.entity_version = ""
        self.name = ""
        self.description = None
        self.fdus = []
        self.virtual_links = []
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        l = []
                        if key == "virtual_links":
                            for i in value:
                                l.append(VirtualNetwork(**i))
                        elif key == "fdus":
                            for i in value:
                                l.append(FDUDescriptor(**i))
                        self.__dict__.update({key: l})
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})

    def as_dict(self):
        d = self.__dict__
        for k, v in d.items():
            if isinstance(v, list) and len(v) > 0:
                value_list = []
                if isinstance(v[0], VirtualNetwork) or isinstance(v[0], FDUDescriptor):
                    for i in v:
                        value_list.append(i.as_dict())
                else:
                    value_list = v
                d.update({k: value_list})
        return d

    def serialize_cbor(self):
        return cbor2.dumps(self.as_dict())

    def serialize_json(self):
        return json.dumps(self.as_dict())

    def serialize_yaml(self):
        return yaml.dump_all(self.as_dict())

    @staticmethod
    def deserialize_yaml(serialized):
        data = yaml.full_load(serialized)
        return EntityDescriptor(**data)

    @staticmethod
    def deserialize_json(serialized):
        data = json.loads(serialized)
        return EntityDescriptor(**data)

    @staticmethod
    def deserialize_cbor(serialized):
        data = cbor2.loads(serialized)
        return EntityDescriptor(**data)


# Ṛecord

class EntityStatus(Enum):
    ONBOARDING = "ONBOARDING"
    ONBOARDED = "ONBOARDED"
    STARTING = "STARTING"
    RUNNING = "RUNNING"
    STOPPING = "STOPPING"
    STOPPED = "STOPPED"
    OFFLOADING = "OFFLOADING"
    OFFLOADED = "OFFLOADED"
    ERROR = "ERROR"
    RECOVERING = "RECOVERING"


class EntityRecord(object):
    def __init__(self, **kwargs):
        self.uuid = None
        self.id = None
        self.status = EntityStatus.ONBOARDING
        self.fdus = []
        self.virtual_links = []
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        value_list = []
                        if key == "fdus":
                            for i in value:
                                value_list.append(uuid.UUID(bytes=i))
                        elif key == "virtual_links":
                            for i in value:
                                value_list.append(uuid.UUID(bytes=i))
                        self.__dict__.update({key: value_list})
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        elif key == "id":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

    def serialize_cbor(self):
        return cbor2.dumps(self.as_dict())

    def serialize_json(self):
        return json.dumps(self.as_dict())

    def serialize_yaml(self):
        return yaml.dump_all(self.as_dict())

    @staticmethod
    def deserialize_yaml(serialized):
        data = yaml.full_load(serialized)
        return EntityRecord(**data)

    @staticmethod
    def deserialize_json(serialized):
        data = json.loads(serialized)
        return EntityRecord(**data)

    @staticmethod
    def deserialize_cbor(serialized):
        data = cbor2.loads(serialized)
        return EntityRecord(**data)
