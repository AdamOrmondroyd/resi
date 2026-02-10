import numpy as np
import pypolychord
from pypolychord.priors import UniformPrior
try:
    from mpi4py import MPI
except ImportError:
    pass
import time
import resi

residr2 = resi.Likelihood(
    "/Users/adam/phd/jesi/jesi/likelihoods/data/desidr2/desidr2_mean.txt",
    "/Users/adam/phd/jesi/jesi/likelihoods/data/desidr2/desidr2_cov.txt",
)


nDims = 2
nDerived = 0


def likelihood(theta):
    h0rd, omegam = theta
    return residr2(h0rd, omegam)


prior = UniformPrior(np.array([3650.0, 0.01]), np.array([18250.0, 0.99]))

paramnames = [('h0rd', r'H_0r_{\mathrm{d}}'), ('omegam', r'\Omega_{\mathrm{m}}')]

tick = time.time()
output = pypolychord.run(
    likelihood,
    nDims,
    nDerived=nDerived,
    prior=prior,
    file_root='residr2',
    nlive=200,
    do_clustering=True,
    read_resume=False,
    paramnames=paramnames,
)
tock = time.time()
print(f"PolyChord run took {tock - tick:.2f} seconds.")

try:
    from anesthetic import make_2d_axes
    fig, ax = make_2d_axes(['h0rd', 'omegam'])
    output.plot_2d(ax)
    fig.savefig('chains/residr2.pdf')
except ImportError:
    print("Install anesthetic for plotting examples.")
