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

## License

`auc-calculator` is available under the terms of the

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your choosing.
