from enum import Enum
import cbor2
import uuid
import json
import yaml

class MigrationKind(Enum):
    LIVE = "LIVE"
    COLD = "COLD"

class ConfigurationKind(Enum):
    SCRIPT = "SCRIPT"
    ENV = "ENV"
    CLOUD_INIT = "CLOUD_INIT"

class InterfaceKind(Enum):
    VIRTUAL = "VIRTUAL"
    WLAN = "WLAN"
    BLUETOOTH = "BLUETOOTH"

class VirtualInterfaceKind(Enum):
    PARAVIRT = "PARAVIRT",
    PCI_PASSTHROUGH = "PCI_PASSTHROUGH"
    SR_IOV = "SR_IOV"
    VIRTIO = "VIRTIO"
    E1000 = "E1000"
    RTL8139 = "RTL8139"
    PCNET = "PCNET"
    BRIDGED = "BRIDGED"
    PHYSICAL = "PHYSICAL"


class StorageKind(Enum):
    BLOCK = "BLOCK"
    OBJECT = "OBJECT"


class Position(object):
    def __init__(self, **kwargs):
        self.lat = ""
        self.lon = ""
        self.radius = 0.0
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__


class Proximity(object):
    def __init__(self, **kwargs):
        self.neighbor = ""
        self.radius = 0.0
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})
        
    def as_dict(self):
        return self.__dict__


class Configuration(object):
    def __init__(self, **kwargs):
        self.conf_kind = None
        self.script = None
        self.env = None
        self.ssh_keys = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

class Image(object):
    def __init__(self, **kwargs):
        self.uuid = None
        self.name = None
        self.uri = ""
        self.checksum = ""
        self.format = ""
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

class ComputationalRequirements(object):
    def __init__(self, **kwargs):
        self.cpu_arch = ""
        self.cpu_min_freq = 0
        self.cpu_min_count = 1
        self.gpu_min_count = 0
        self.fpga_min_count = 0
        self.operating_system = None
        self.ram_size_mb = 0
        self.storage_size_mb = 0
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

class GeographicalRequirements(object):
    def __init__(self, **kwargs):
        self.position = None
        self.proximity = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})
    
    def as_dict(self):
        return self.__dict__

class VirtualInterface(object):
    def __init__(self, **kwargs):
        self.vif_kind = VirtualInterfaceKind.VIRTIO
        self.parent = None
        self.bandwidth = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})
    def as_dict(self):
        return self.__dict__

class ConnectionPointDescriptor(object):
    def __init__(self,**kwargs):
        self.uuid = None
        self.name = ""
        self.id = ""
        self.vdl_ref = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})
    def as_dict(self):
        return self.__dict__

class Interface(object):
    def __init__(self, **kwargs):
        self.name = ""
        self.kind = InterfaceKind.VIRTUAL
        self.mac_address = None
        self.virtual_interface = VirtualInterface()
        self.cp_id = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        if key == "virtual_interface":
                            self.__dict__.update({key: VirtualInterface(**value)})
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})
        
    def as_dict(self):
        d = self.__dict__
        d.update({"virtual_interface":d['virtual_inteface'].as_dict()})
        return d

class StorageDescriptor(object):
    def __init__(self, **kwargs):
        self.id = ""
        self.storage_kind = StorageKind.BLOCK
        self.size = 0
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__



class FDUDescriptor(object):
    def __init__(self,**kwargs):
        self.uuid = None
        self.id = ""
        self.name = ""
        self.version = ""
        self.fdu_version = ""
        self.descriptor = None
        self.hypervisor = ""
        self.image = None
        self.hypervisor_specific = None
        self.computation_requirements = ComputationalRequirements()
        self.grographical_requirements = None
        self.interfaces = []
        self.storage = []
        self.connection_points = []
        self.migration_kind = MigrationKind.COLD
        self.replicas = None
        self.depends_on = []
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        if key == "computation_requirements":
                            self.__dict__.update({key: ComputationalRequirements(**value)})
                        elif key == "geographical_requirements":
                            self.__dict__.update({key: GeographicalRequirements(**value)})
                        elif key == "configuration":
                            self.__dict__.update({key: Configuration(**value)})
                        elif key == "image":
                            self.__dict__.update({key: Image(**value)})
                    elif isinstance(value, list):
                        l = []
                        if key == "interfaces":
                            for i in value:
                                l.append(Interface(**i))
                        elif key == "storage":
                            for i in value:
                                l.append(StorageDescriptor(**i))
                        elif key == "connection_points":
                            for i in value:
                                l.append(ConnectionPointDescriptor(**i))
                        elif key == "depends_on":
                            l = value
                        self.__dict__.update({key: l})
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})

    def as_dict(self):
        d = self.__dict__
        for k, v in d.items():
            if isinstance(v, GeographicalRequirements):
                d.update({k: v.as_dict()})
            elif isinstance(v, ComputationalRequirements):
                d.update({k: v.as_dict()})
            elif isinstance(v, Image):
                d.update({k: v.as_dict()})
            elif isinstance(v, Configuration):
                d.update({k: v.as_dict()})
            elif isinstance(v, list) and len(v) > 0:
                l = []
                if isinstance(v[0], Interface) or isinstance(v[0], StorageDescriptor) or isinstance(v[0], ConnectionPointDescriptor):
                    for i in v:
                        l.append(i.as_dict())
                else:
                    l = v
                d.update({k: l})
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
        return FDUDescriptor(**data)


    @staticmethod
    def deserialize_json(serialized):
        data = json.loads(serialized)
        return FDUDescriptor(**data)

    @staticmethod
    def deserialize_cbor(serialized):
        data = cbor2.loads(serialized)
        return FDUDescriptor(**data)
        

# Records

class FDUState(Enum):
    DEFINED = "DEFINED"
    CONFIGURED = "CONFIGURED"
    RUNNING = "RUNNING"
    ERROR = "ERROR"


class RecordVirtualInterface(object):
    def __init__(self, **kwargs):
        self.vif_kind = VirtualInterfaceKind.VIRTIO
        self.bandwidth = None
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

class RecordInterface(object):
    def __init__(self, **kwargs):
        self.name = ""
        self.kind = InterfaceKind.VIRTUAL
        self.mac_address = None
        self.virtual_interface = RecordVirtualInterface()
        self.cp_uuid = None
        self.intf_uuid = ""
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        if key == "virtual_interface":
                            self.__dict__.update({key: RecordVirtualInterface(**value)})
                    elif isinstance(value, list):
                        pass
                    else:
                        if key == "cp_uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        elif key == "intf_uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})
        
    def as_dict(self):
        d = self.__dict__
        d.update({"virtual_interface":d['virtual_inteface'].as_dict()})
        return d




class RecordConnectionPoint(object):
    def __init__(self, **kwargs):
        self.uuid = ""
        self.id = ""
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        pass
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})

    def as_dict(self):
        return self.__dict__

class FDURecord(object):
    def __init__(self,**kwargs):
        self.uuid = ""
        self.fdu_uuid = ""
        self.node = ""
        self.interfaces = []
        self.connection_points = []
        self.status = FDUState.DEFINED
        self.error = None
        self.hypervisor_specific = None
        self.restarts = 0
        if kwargs is not None:
            for key, value in kwargs.items():
                if hasattr(self, key):
                    if isinstance(value, dict):
                        pass
                    elif isinstance(value, list):
                        l = []
                        if key == "interfaces":
                            for i in value:
                                l.append(RecordConnectionPoint(**i))
                        elif key == "connection_points":
                            for i in value:
                                l.append(RecordConnectionPoint(**i))
                        self.__dict__.update({key: l})
                    else:
                        if key == "uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        elif key == "fdu_uuid":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        elif key == "node":
                            self.__dict__.update({key: uuid.UUID(bytes=value)})
                        else:
                            self.__dict__.update({key: value})


    def as_dict(self):
        d = self.__dict__
        for k, v in d.items():
            if isinstance(v, list) and len(v) > 0:
                l = []
                if isinstance(v[0], RecordConnectionPoint) or isinstance(v[0], RecordConnectionPoint):
                    for i in v:
                        l.append(i.as_dict())
                else:
                    l = v
                d.update({k: l})
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
        return FDURecord(**data)

    @staticmethod
    def deserialize_json(serialized):
        data = json.loads(serialized)
        return FDURecord(**data)

    @staticmethod
    def deserialize_cbor(serialized):
        data = cbor2.loads(serialized)
        return FDURecord(**data)