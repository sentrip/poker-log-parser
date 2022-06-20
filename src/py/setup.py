import os
from setuptools import setup, Extension

rust_proj_dir = os.path.abspath('../..')
cur_dir = os.getcwd()
os.chdir(rust_proj_dir)
os.system('cargo build --release')
os.chdir(cur_dir)
rust_lib_dir = os.path.join(rust_proj_dir, 'target', 'release')


mod = Extension(
    'pklp', 
    sources = ['main.c'],
    libraries = ['pklp', 'bcrypt', 'Ws2_32', 'UserEnv', 'advapi32', 'user32'],
    library_dirs = [rust_lib_dir],
)

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()


setup(
    name = 'pklp',
    version = '0.0.1',
    author='Djordje Pepic',
    author_email='djordje.pepic.10@gmail.com',
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/sentrip/poker-log-parser",
    description = 'A blazingly-fast library for converting poker hand history text files into JSON',
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ]
    packages = ['pklpy'],
    ext_modules = [mod],
    zip_safe=False
)
