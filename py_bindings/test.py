import os
os.chdir('build/lib')

import gol_py

s = gol_py.setup()

print(s)

gol_py.step(s, ntimes=100)

print(s)
