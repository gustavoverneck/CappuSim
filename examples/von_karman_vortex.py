# Von-Kármán Vortex
from CappuSim.lbm import LBM
from CappuSim.utils import Config

def VKV(x, y, z, u, rho, flags):
    pass
    

def main():
    config = Config(velocities_set="D2Q9", grid_size=(1024, 1024, 1), viscosity=0.1)
    lbm = LBM(config)
    lbm.setInitialConditions()
    lbm.run(1000)