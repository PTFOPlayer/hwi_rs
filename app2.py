
#program to display the stats of a pc using the pyqt5 library
from cProfile import label
import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *
import processes
from time import sleep

class Window(QWidget):
      
    def __init__(self):
        super().__init__()
        self.setGeometry(100, 100, 800, 400)
        layout = QVBoxLayout()
        # creating a label for cpu usage
        self.label_cpu = QLabel()
        self.label_cpu.setAlignment(Qt.AlignLeft )
        # creating a label for ram usage
        self.label_ram = QLabel()
        self.label_ram.setAlignment(Qt.AlignLeft )
        # creating a label for gpu mem usage
        self.label_gpu_mem = QLabel()
        self.label_gpu_mem.setAlignment(Qt.AlignLeft )
        # creating a label for gpu usage
        self.label_gpu = QLabel()
        self.label_gpu.setAlignment(Qt.AlignLeft )
        
        layout.addWidget(self.label_cpu)
        layout.addWidget(self.label_ram)
        layout.addWidget(self.label_gpu)
        layout.addWidget(self.label_gpu_mem)
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
        self.label_cpu.setText("CPU usage : " + str(processes.cpu_monit()[0]) + "%"
                               + "\nCPU cores : " + str(processes.cpu_monit()[1]) 
                               + "\nCPU threads : " + str(processes.cpu_monit()[2]))
        # showing ram usage in the label
        self.label_ram.setText("RAM max : " + str(processes.ram_monit()[0]) + "GB" +
                               "\nRAM used : " + str(processes.ram_monit()[1]) + "GB"
                               "\nRAM usage : " + str(processes.ram_monit()[2]) + "%")
        # showing gpu mem usage in the label
        gpu_mem = processes.nvd_gpumem()
        self.label_gpu_mem.setText("GPU memory used : " + str(gpu_mem[0]) + "MB" +
                               "\nGPU memory max : " + str(gpu_mem[1]) + "MB")
        # showing gpu usage in the label
        gpu_usg = processes.nvd_gpuusg()
        self.label_gpu.setText("\n GPU model : " + str(gpu_usg[0]) + "\n"
                                + "GPU temperature in : " + str(gpu_usg[2]) + " °C \n"
                                + "GPU power in W : " + str(gpu_usg[3])
                                + "\n GPU usage : " + str(gpu_usg[1]) + "% \n")
  
# create pyqt5 app
App = QApplication(sys.argv)
# create the instance of our Window
window = Window()
# showing all the widgets
window.show()
# start the app
App.exit(App.exec_())
