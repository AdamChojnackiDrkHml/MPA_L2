import numpy as np
from pandas import read_csv
from matplotlib import pyplot as plt

def plot(path, suff):
    arr = np.loadtxt(path, delimiter=";")
    arr = arr.transpose()
    X = range(200, 5000, 40)

    plt.plot(X, arr[0], label="Kruskal")
    plt.plot(X, arr[1], label="Prim")
    plt.xlabel("N")
    plt.ylabel("Time")
    plt.legend()
    plt.savefig(f"plots/Comp_{suff}")
    plt.clf()

    plt.plot(X, arr[2])
    plt.savefig(f"plots/MST_{suff}")
    plt.clf()
    
plot("data/resultTestUni.csv", "Uni")
plot("data/resultTestNorm.csv", "Norm")