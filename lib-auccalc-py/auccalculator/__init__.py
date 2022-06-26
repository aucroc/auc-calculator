# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License or MIT License

import _auccalculator
from . import metrics

__all__ = "__version__", "Confusion", "metrics"

__version__ = _auccalculator.__version__
Confusion = _auccalculator.Confusion
