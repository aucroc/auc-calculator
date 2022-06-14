# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License

"""
Thin wrapper around the Rust auccalc module.
"""

from .auccalc import ConfusionPy

class Confusion:
    """Confusion object for calculating AUC metrics.
    """

    def __init__(self, y_pred, y_true):
        self.confusion = ConfusionPy(y_pred, y_true)

    def aucpr(self, min_recall=0.0):
        """Calculate the area under the precision-recall curve.

        Arguments:
            min_recall: Minimum recall to use in [0.0 - 1.0]

        Returns:
            Area under the precision-recall curve.
        """
        return self.confusion.aucpr(min_recall)

    def aucroc(self):
        """Calculate the area under the receiver operating characteristic curve.

        Returns:
            Area under the receiver operating characteristic curve.
        """
        return self.confusion.aucroc()
