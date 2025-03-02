import matplotlib.pyplot as plt

from CappuSim.lbm import LBM
from CappuSim.utils import Config

config = Config(velocities_set="D3Q19", grid_size=(512, 512, 512), viscosity=0.1).get()

lbm = LBM(config)
lbm.run(1000)