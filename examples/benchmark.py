import matplotlib.pyplot as plt

from CappuSim.lbm import LBM
from CappuSim.utils import Config

config = Config(velocities_set="D2Q9", grid_size=(1024, 1024, 1), viscosity=0.1).get()

lbm = LBM(config)
lbm.run(1000)