
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
    if ntimes == 1:
        data = json.dumps(list(board))
        resp = requests.post(RPC_URL+'/step/', data=data)
    else:
        data = json.dumps((ntimes, list(board)))
        resp = requests.post(RPC_URL+'/step/many/', data=data)
    new = set(tuple(x) for x in json.loads(resp.content.decode()))
    board.clear()
    board.update(new)

def test():
    a,b = rpc_setup(), gol_py.setup()
    assert(a == b), 'bad setup!'

    rpc_step(a)
    gol_py.step(b)
    assert(a == b), 'bad step!'

    rpc_step(a,ntimes=100)
    gol_py.step(b, ntimes=100)
    assert(a == b), 'bad step many!'

def bench():
    print('SETUP')
    timer(rpc_setup, 1000)
    timer(gol_py.setup, 1000)

    print('STEP')
    a, b = rpc_setup(), gol_py.setup()
    timer(lambda: rpc_step(a), 1000)
    timer(lambda: gol_py.step(b), 1000)

    print('STEP 100')
    timer(lambda: rpc_step(a, ntimes=100), 100)
    timer(lambda: gol_py.step(b, ntimes=100), 100)

    print('STEP 1000')
    timer(lambda: rpc_step(a, ntimes=1000), 10)
    timer(lambda: gol_py.step(b, ntimes=1000), 10)

    assert (a == b)

def timer(fn, n):
    print(timeit.timeit(fn, 'gc.enable()', number=n))
