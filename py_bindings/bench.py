
import requests
import json

import os
os.chdir('build/lib')
import gol_py

RPC_URL = 'http://localhost:3000/'

def rpc_setup():
    board = requests.get(RPC_URL+'setup/')
    return set(x for x in json.loads(board.content.decode()))


