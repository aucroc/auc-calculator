Work-in-progress: Rust re-implementation of the Davis/Goadrich AUC Calculator, hopefully
making it easier to distribute native code.

## Build the package + Python extension

This builds with Cargo + maturin + setuptools-rust currently.

```console
pip install maturin setuptools-rust
```

Create a development build:

```console
cd lib-auccalc-py/
maturin develop --release
pip install -e .
```

This implements a scikit-learn-style metrics class that takes `y_true` and `y_pred` vectors. 

```python
>>> from auccalculator.metrics import roc_auc_score
>>> roc_auc_score(
...   [1, 1, 0, 1, 1, 1, 0, 0, 1, 0],
...   [0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505],
... )
0.75
```

Or `Confusion` instances can be initialized to compute AUC-ROC and AUC-PR:

```python
>>> from auccalculator import Confusion
>>> conf = Confusion(
...   [1, 1, 0, 1, 1, 1, 0, 0, 1, 0],
...   [0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505],
... )
>>> conf.aucroc()
0.75
>>> conf.aucpr()
0.8243055555555555
```

## License

`auc-calculator` is available under the terms of the

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your choosing.
