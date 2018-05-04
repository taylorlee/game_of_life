
import requests
import json
import timeit

import os
os.chdir('build/lib')
import gol_py

RPC_URL = 'http://localhost:3000'

def rpc_setup():
    resp = requests.get(RPC_URL+'/setup/')
    return set(tuple(x) for x in json.loads(resp.content.decode()))

def rpc_step(board, ntimes=1):
    data = json.dumps((ntimes, list(board)))
    resp = requests.post(RPC_URL+'/step/', data=data)
    board.clear()
    board.update(set(tuple(x) for x in json.loads(resp.content.decode())))

def bench(gc=False):
    print('SETUP')
    output(rpc_setup, gol_py.setup, 1000, gc=gc)

    a = rpc_setup()
    b = gol_py.setup()

    for i in range(4):
        time_step(a, b, 10**i)

    assert (a == b)

def time_step(a,b,n, gc=False):
    loops = 10000 // n
    print('\nSTEP: {} loops of step_{}'.format(loops, n))
    output(
        lambda: rpc_step(a,ntimes=n),
        lambda: gol_py.step(b, ntimes=n),
        loops,
        gc=gc,
    )

def output(f1, f2, loops, gc=False):
    gc = 'gc.enable()' if gc else ''
    print('rpc:  ', round(timeit.timeit(f1, number=loops), 4))
    print('pyo3: ', round(timeit.timeit(f2, number=loops), 4))

if __name__ == '__main__':
    print('WITH GC')
    bench(gc=True)
    print('WITHOUT GC')
    bench(gc=False)

