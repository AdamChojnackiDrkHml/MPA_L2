import numpy as np
from pandas import read_csv
from matplotlib import pyplot as plt

arr = np.loadtxt("data/resultTest.csv", delimiter=";")
arr = arr.transpose()
X = range(100, 1000, 25)

plt.plot(X, arr[0], label="Kruskal")
plt.plot(X, arr[1], label="Prim")
plt.xlabel("N")
plt.ylabel("Time")
plt.legend()
plt.savefig("plots/Comp")
plt.clf()

plt.plot(X, arr[2])
plt.savefig("plots/MST")