
#program to display the stats of a pc using the pyqt5 library
#author: WhiskyAKM (Patryk T)
#
#app works with the following os:
#windows 10 - partial support (only cpu cores and usage, ram, drives, network, nvidia gpu)
#linux - full support (cpu cores, usage, ram, drives, network, nvidia gpu)
#currently only works with nvidia gpu but can be extended to other gpu's
#
#radeon gpu's will be supported in the future by using the rocm-smi 
#usage of rocm may cause reduction of stats refresh rate and will cause large ram usage
#
#supported gpu's:
#
#nvidia gpu's: 
#full support for all gpus newer than tesla 2.0 
#tested on series: maxwell, pascal, turing 
#radeon gpu's:
#ROCM supports gpu's from GCN 1.0 to RDNA2
#intel gpu's:
#ROCM teoretically supports all intel gpu's from Igpu's from Intel core Gen4 to Xe series
#intel gpu's support may be extended whem intel launches Xe series gpu's
import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *
import processes
import json
import cpuinfo

with open('config.json') as configf:
        #read json file
        config = json.load(configf)
        

class Window(QWidget):
      
    def __init__(self):
        super().__init__()
        self.setGeometry(100, 100, 800, 400)
        layout = QVBoxLayout()
        
        # setting font style
        fontcolor = config.get('font_color')
        fontsize = config.get('font_size')
        fontfamily = config.get('font_family')
        FontStyle = f"font-size: {fontsize}; color: {fontcolor}; font-family: {fontfamily};"
        
        # creating a label for cpu usage
        self.label_cpu = QLabel()
        self.label_cpu.setStyleSheet(FontStyle)
        self.label_cpu.setAlignment(Qt.AlignLeft)
        
        # creating a label for ram usage
        self.label_ram = QLabel()
        self.label_ram.setStyleSheet(FontStyle)
        self.label_ram.setAlignment(Qt.AlignLeft)
        
        # creating a label for gpu mem usage
        self.label_gpu_mem = QLabel()
        self.label_gpu_mem.setStyleSheet(FontStyle)
        self.label_gpu_mem.setAlignment(Qt.AlignLeft)
        
        # creating a label for gpu usage
        self.label_gpu = QLabel()
        self.label_gpu.setStyleSheet(FontStyle)
        self.label_gpu.setAlignment(Qt.AlignLeft )
        
        #creating a label for network usage
        self.label_network = QLabel()
        self.label_network.setStyleSheet(FontStyle)
        self.label_network.setAlignment(Qt.AlignLeft)
        
        #creating a labwl for drive usage
        self.label_drive = QLabel()
        self.label_drive.setStyleSheet(FontStyle)
        self.label_drive.setAlignment(Qt.AlignLeft)
        
        # setting progress bar style
        barcolor = config.get('bar_color')
        bar_style = ("QProgressBar::chunk "
                    "{"
                    f"background-color: {barcolor};"
                    "}")
        
        #label for cpu name
        self.label_cpu_const = QLabel()
        self.label_cpu_const.setStyleSheet(FontStyle)
        self.label_cpu_const.setAlignment(Qt.AlignLeft)
        layout.addWidget(self.label_cpu_const)
        self.label_cpu_const.setText("CPU name : " + cpuinfo.get_cpu_info()['brand_raw'] + 
                                    "\n \nCPU cores : " + str(processes.cpu_const()[0]) 
                                    + "\nCPU threads : " + str(processes.cpu_const()[1]))
        
        #progress bar for cpu usage
        self.progressBar_cpu = QProgressBar()
        self.progressBar_cpu.setStyleSheet(bar_style)
        self.progressBar_cpu.setValue(0)
        self.progressBar_cpu.setMaximum(100)
        self.progressBar_cpu.setAlignment(Qt.AlignCenter)
        
        #progress bar for ram usage
        self.progressBar_ram = QProgressBar()
        self.progressBar_ram.setStyleSheet(bar_style)
        self.progressBar_ram.setValue(0)
        self.progressBar_ram.setMaximum(100)
        self.progressBar_ram.setAlignment(Qt.AlignCenter)
        
        #progress bar for gpu usage
        self.progressBar_gpu = QProgressBar()
        self.progressBar_gpu.setStyleSheet(bar_style)
        self.progressBar_gpu.setValue(0)
        self.progressBar_gpu.setMaximum(100)
        self.progressBar_gpu.setAlignment(Qt.AlignCenter)
        
        # layout
        layout.addWidget(self.label_cpu)
        layout.addWidget(self.progressBar_cpu)
        layout.addWidget(self.label_ram)
        layout.addWidget(self.progressBar_ram)
        layout.addWidget(self.label_gpu)
        layout.addWidget(self.progressBar_gpu)
        layout.addWidget(self.label_gpu_mem)
        layout.addWidget(self.label_network)
        layout.addWidget(self.label_drive)
        
        self.setLayout(layout)
        
        # creating a timer object
        timer = QTimer(self)
        # adding action to timer
        timer.timeout.connect(self.update)
        # update the timer every second
        timer.start(1000)
  
    # method called by timer
    def update(self):
        # showing cpu usage in the label
        cpu_usg = processes.cpu_monit()
        self.label_cpu.setText("\nCPU temperature : "+str(cpu_usg[1]) + "°C"
                               + "\n CPU frequency : " + str(cpu_usg[2]) + "GHz"
                               + "\nCPU usage :")
        # updating the progress bar
        bar_cpu = int(cpu_usg[0])
        self.progressBar_cpu.setValue(bar_cpu)
        
        # showing ram usage in the label
        ram_usg = processes.ram_monit()
        self.label_ram.setText("\nRAM max : " + str(ram_usg[0]) + "GB" +
                               "\nRAM used : " + str(ram_usg[1]) + "GB"
                               "\nRAM usage :")
        # updating the progress bar
        bar_ram = int(ram_usg[2])
        self.progressBar_ram.setValue(bar_ram)
        
        # showing gpu usage in the label
        try:
            gpu_usg = processes.nvd_gpuusg()
            self.label_gpu.setText("\nGPU model : " + str(gpu_usg[0]) + "\n"
                                    + "GPU temperature in : " + str(gpu_usg[2]) + " °C \n"
                                    + "GPU power in W : " + str(gpu_usg[3])
                                    + "\nGPU usage :")
            bar_gpu = int(gpu_usg[1])
            self.progressBar_gpu.setValue(bar_gpu)
            gpu_mem = processes.nvd_gpumem()
            self.label_gpu_mem.setText("GPU memory used : " + str(gpu_mem[0]) + "MB" +
                                   "\nGPU memory max : " + str(gpu_mem[1]) + "MB")
        except:
            try:
                gpu_temp = processes.amd_gpuusg()
                gpu_usg = gpu_temp[0]
                gpu_mem = gpu_temp[1]
                self.label_gpu.setText("\n GPU usage : " + str(gpu_usg))
                bar_gpu = int(gpu_usg)
                self.progressBar_gpu.setValue(bar_gpu)
                self.label_gpu_mem.setText("GPU memory used : " + str(gpu_mem))
            except:
                self.label_gpu.setText("\nGPU model : " + "Not found")
                self.progressBar_gpu.setValue(0)
        # showing network usage in the label
        self.label_network.setText("\nNetwork usage : " + str(processes.network_usage()) + "Mb/s")
        
        # showing drive usage in the label
        try:
            self.label_drive.setText("\nDrive Fullness : " + str(processes.drives_monit_linux()) + " %")
        except:
            self.label_drive.setText("\nDrive Fullness : " + str(processes.drives_monit_win())+ " %")
        
        
# create pyqt5 app
App = QApplication(sys.argv)
# create the instance of our Window
window = Window()
window.setWindowTitle("PC stats")
bg = config.get("background")
window.setStyleSheet(f"background-color: {bg};")

# showing all the widgets
window.show()
# start the app
App.exit(App.exec_())
