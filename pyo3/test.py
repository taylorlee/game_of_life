import os

os.system('python3 setup.py build')
os.chdir('build/lib')

import gol_py

board = gol_py.setup()
gol_py.step(board, ntimes=1000)

assert(len(board) == 457)

print('Computation is correct!')

print('Benchmark:')
import timeit
print(timeit.timeit(lambda: gol_py.step(gol_py.setup(), ntimes=1000), number=10))
