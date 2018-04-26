from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='gol_py',
    version='1.0',
    rust_extensions=[
        RustExtension(
            'gol_py._gol',
            'Cargo.toml',
            binding=Binding.PyO3
        )
    ],
    packages=['gol_py'],
    zip_safe=False
)
