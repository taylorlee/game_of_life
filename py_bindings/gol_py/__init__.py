from . import _gol


def setup():
    container = set()
    _gol.setup(container)
    return container


def step(board, ntimes=1):
    _gol.step(board, ntimes)
