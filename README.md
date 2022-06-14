Work-in-progress: Rust re-implementation of the Davis/Goadrich AUC Calculator, hopefully
making it easier to distribute native code.

## Build the package + Python extension

This builds with Cargo + maturin currently.

```console
pip install maturin
```

Create a development build:

```console
cd lib-auccalc-py/
maturin develop --release
```

Check that it's available:

```python
from auccalc.auccalc import ConfusionPy
```
