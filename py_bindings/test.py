
import os
import timeit

os.system('python3 setup.py build')

os.chdir('build/lib')

import gol_py

board = gol_py.setup()

ncells = len(board)
assert(ncells == 7), ncells

gol_py.step(board, ntimes=1000)

ncells = len(board)
assert(ncells == 457), ncells

print('computation is correct!')
print('Benchmark:')
print(timeit.timeit(lambda: gol_py.step(gol_py.setup(), ntimes=1000), number=10))
