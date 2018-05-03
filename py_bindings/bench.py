
import requests
import json

import os
os.chdir('build/lib')
import gol_py

RPC_URL = 'https://localhost:3000/'

def rpc_setup():
    board = requests.get(RPC_URL+'setup/')
    return set(json.loads(board.content))


