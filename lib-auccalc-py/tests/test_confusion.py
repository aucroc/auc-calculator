# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License or MIT License

from auccalculator import Confusion
import pytest


def test_confusion_auc_roc_auc_pr_1():
    conf = Confusion(
        [1, 1, 0, 1, 1, 1, 0, 0, 1, 0],
        [0.9, 0.8, 0.7, 0.6, 0.55, 0.54, 0.53, 0.52, 0.51, 0.505],
    )
    assert conf.aucpr() == pytest.approx(0.8243055555555555, rel=0.0001)
    assert conf.aucroc() == pytest.approx(0.75, rel=0.0001)
