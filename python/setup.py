import sys
import subprocess
import os
from setuptools import setup, find_packages
from setuptools.command.install import install
from setuptools.command.develop import develop


class CustomInstallCommand(install):
    def run(self):
        # Update pip before installation
        subprocess.check_call([sys.executable, "-m", "pip", "install", "--upgrade", "pip"])
        install.run(self)


class CustomDevelopCommand(develop):
    def run(self):
        # Update pip before installation
        subprocess.check_call([sys.executable, "-m", "pip", "install", "--upgrade", "pip"])
        develop.run(self)


setup(
    py_modules=["life", "piano", "game_board", "main", "config", "update_pip"],
    package_dir={"": "."},
    packages=find_packages(where="."),
    package_data={
        "": ["*.py", "*.toml", "*.sh", "*.bat", "*.env"],
    },
    include_package_data=True,
    cmdclass={
        'install': CustomInstallCommand,
        'develop': CustomDevelopCommand,
    },
)