# Copyright © 2022 Alexander L. Hayes
# Apache 2.0 License

from .pyauc import Confusion

confusion = Confusion([0.8, 0.7, 0.6, 0.3, 0.7], [1, 1, 1, 0, 0])

print(confusion.aucpr())
print(confusion.aucroc())
