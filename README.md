[![Gitter](https://badges.gitter.im/atolab/fog05.svg)](https://gitter.im/atolab/fog05?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)
[![License](https://img.shields.io/badge/License-EPL%202.0-blue)](https://choosealicense.com/licenses/epl-2.0/)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Documentation Status](https://readthedocs.org/projects/eclipse-fog05-python3-client-api/badge/?version=latest)](https://eclipse-fog05-python3-client-api.readthedocs.io/en/latest/?badge=latest)

# Eclispe fog05 Python API

This repository contains the Eclipse fog05 Client API for Python

With this API you can interact with Eclipse fog05 nodes and deploy your application components


-------------------------------
## How to install it

<!-- The Eclipse fog05-python library is available on [Pypi.org](https://pypi.org/project/eclipse-fog05/).
Install the latest available version using `pip`:
```
pip install eclipse-fog05
``` -->

:warning:WARNING:warning: fog05-python is developped in Rust.
On Pypi.org we provide binary wheels for the most common platforms (MacOS, Linux x86). But also a source distribution package for other platforms.
However, for `pip` to be able to build this source distribution, there some prerequisites:
 - `pip` version 19.3.1 minimum (for full support of PEP 517).
   (if necessary upgrade it with command: `'sudo pip install --upgrade pip'` )
 - Have a Rust toolchain installed (instructions at https://rustup.rs/)

### Supported Python versions and platforms

fog05-python has been tested with Python 3.6, 3.7, 3.8 and 3.9.

It relies on the [fog05](https://github.com/eclipse-fog05/fog05/tree/master/fog05-sdk) Rust API which require the full `std` library. See the list Rust supported platforms here: https://doc.rust-lang.org/nightly/rustc/platform-support.html .

Currently only the following platforms have been tested:
 * Linux
 * MacOS X


-------------------------------
## How to build it

Requirements:
 * Python >= 3.5
 * A virtual environement such as [venv](https://docs.python.org/3/library/venv.html), [miniconda](https://docs.conda.io/en/latest/miniconda.html) or [Conda](https://docs.conda.io/projects/conda/en/latest/)
 * [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
   Then install the __*nighlty*__ toolchain running:
   ```bash
   rustup toolchain install nightly
   ```
 * [maturin](https://github.com/PyO3/maturin). Install it with:
   ```bash
   pip install maturin
   ```

Steps:
 * Make sure your shell is running in a Python virtual environment.
    ```bash
    python3 -m venv ../venv
    source ../venv/bin/activate
    ```
 * Build fog05-python using **maturin**
   ```bash
   cd fog05-python
   maturin develop --release
   ```

Maturin will automatically build the fog05 Rust API, as well as the fog05-python API and install it in your Python virtual environement.


## Docs

Documentation is available here: [readthedocs.io](https://eclipse-fog05-python3-client-api.readthedocs.io/en/latest/?badge=latest)

## Examples

Examples are available in the [examples](https://github.com/eclipse-fog05/examples/tree/master/fim_api/python) repository
