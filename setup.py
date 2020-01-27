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
# Contributors: Gabriele Baldoni, ADLINK Technology Inc.
# OCaml implementation and API

#!/usr/bin/env python3

from setuptools import setup

setup(
    name='fog05',
    version='0.1.0',
    python_requires='>=3',
    author='ADLINK',
    packages=['fog05'],
    description='Eclipse fog05 Client API',
    url='https://fog05.io',
    author_email='gabriele.baldoni@adlinktech.com',
    license='Apache 2.O or EPL 2.0',
    install_requires=['fog05-sdk==0.1.0','yaks==0.3.0', 'jsonschema','mvar==0.0.1'],
    scripts=[],
    include_package_data=True
)
