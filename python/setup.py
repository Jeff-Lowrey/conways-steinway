import sys
import subprocess
from setuptools import setup
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
    py_modules=["life", "piano", "config", "game_board", "main", "update_pip"],
    cmdclass={
        'install': CustomInstallCommand,
        'develop': CustomDevelopCommand,
    },
)