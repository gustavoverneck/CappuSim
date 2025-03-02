from CappuSim.lbm import LBM
from CappuSim.utils import Config


def initial_conditions(x, y, z, u, rho, flags):
    if x == 0:
        u[x,y,z] = 0.2
    else:
        u[x,y,z] = 0.1


if __name__ == "__main__":
    config = Config().get()
    lbm = LBM(config)
    lbm.setInitialConditions(initial_conditions)
    lbm.run(1000)