# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License or MIT License

from auccalculator.metrics import roc_auc_score
import pytest


def test_roc_auc_score_1():
    assert roc_auc_score([0, 1, 0], [0.5, 0.6, 0.75]) == pytest.approx(0.75, rel=0.001)
