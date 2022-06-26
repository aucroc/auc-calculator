# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License or MIT License

# Methods for pulling the version number from Cargo.toml is based on the setup.py
# for `rtoml`. Copyright Samuel Colvin, used under the terms of the MIT License.
# https://github.com/samuelcolvin/rtoml

import re
from pathlib import Path

from setuptools import setup
from setuptools_rust import Binding, RustExtension

description = "Methods for computing AUC-ROC and AUC-PR metrics in Rust/Python."

THIS_DIR = Path(__file__).resolve().parent
try:
    long_description = (THIS_DIR / "README.md").read_text()
except FileNotFoundError:
    long_description = description

cargo = Path(THIS_DIR / "Cargo.toml").read_text()
VERSION = re.search('version *= *"(.*?)"', cargo).group(1)

setup(
    name="auccalculator",
    version=VERSION,
    description=description,
    long_description=long_description,
    url="https://github.com/srlearn/auc-calculator",
    author="Alexander L. Hayes (hayesall)",
    author_email="alexander@batflyer.net",
    python_requires=">=3.7",
    rust_extensions=[
        RustExtension("auccalculator._auccalculator", binding=Binding.PyO3)
    ],
    packages=["auccalculator"],
    zip_safe=False,
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Rust",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "License :: OSI Approved :: MIT License",
        "License :: OSI Approved :: Apache Software License",
        "Intended Audience :: Science/Research",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
)
