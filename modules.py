#program to install modules for pcstats program using pip/pip3 and os.system 

from os import system


def install(module):
    system('pip install ' + module)
    print('Installed ' + module)
    
def main_install():
    modules = ['psutil', 'tkinter']
    for module in modules:
        install(module)