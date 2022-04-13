from PyQt5 import QtCore, QtGui, QtWidgets
from  PyQt5.QtCore import QTimer
import processes
import configparser
import cpuinfo

config = configparser.ConfigParser()
config.read('config.ini')

gpuconf = config['gpu']['vendor']

class Ui_MainWindow(object):
    def setupUi(self, MainWindow):
        MainWindow.setObjectName("MainWindow")
        MainWindow.resize(777, 582)
        MainWindow.setAutoFillBackground(True)
        self.centralwidget = QtWidgets.QWidget(MainWindow)
        self.centralwidget.setObjectName("centralwidget")
        
        
        ### FONT ###
        fontcolor = f'color : {config["font"]["color"]}'
        fontsize = int(config["font"]["size"])
        font = QtGui.QFont()
        font.setPointSize(fontsize)
        ### FONT - END ###      
        
        ### BAR ###
        
        barcolor = config['bar']['color']
        bar_style = ("QProgressBar::chunk "
                    "{"
                    f"background-color: {barcolor};"
                    "}")
        ### BAR - END ###
         
        self.Cpu_2 = QtWidgets.QLabel(self.centralwidget)
        self.Cpu_2.setGeometry(QtCore.QRect(10, 60, 1081, 81))
        self.Cpu_2.setFont(font)
        self.Cpu_2.setStyleSheet(fontcolor)
        self.Cpu_2.setText("<html><head/><body><p>CPU Core</p><p>CPU threads</p><p><br/></p></body></html>")
        self.Cpu_2.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Cpu_2.setWordWrap(False)
        self.Cpu_2.setObjectName("Cpu_2")
        
        self.Cpu_3 = QtWidgets.QLabel(self.centralwidget)
        self.Cpu_3.setGeometry(QtCore.QRect(330, 60, 781, 81))
        self.Cpu_3.setFont(font)
        self.Cpu_3.setStyleSheet(fontcolor)
        self.Cpu_3.setText("<html><head/><body><p>CPU temp</p><p>CPU Gh<br/></p></body></html>")
        self.Cpu_3.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Cpu_3.setWordWrap(False)
        self.Cpu_3.setObjectName("Cpu_3")
        
        self.Cpu_1 = QtWidgets.QLabel(self.centralwidget)
        self.Cpu_1.setGeometry(QtCore.QRect(10, 10, 1081, 41))
        self.Cpu_1.setFont(font)
        self.Cpu_1.setStyleSheet(fontcolor)
        self.Cpu_1.setText("<html><head/><body><p>CPU const1<br/></p></body></html>")
        self.Cpu_1.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Cpu_1.setWordWrap(False)
        self.Cpu_1.setObjectName("Cpu_1")
        
        self.Cpu_4 = QtWidgets.QLabel(self.centralwidget)
        self.Cpu_4.setGeometry(QtCore.QRect(10, 160, 781, 31))
        self.Cpu_4.setFont(font)
        self.Cpu_4.setStyleSheet(fontcolor)
        self.Cpu_4.setText("<html><head/><body><p>CPU USG</p></body></html>")
        self.Cpu_4.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Cpu_4.setWordWrap(False)
        self.Cpu_4.setObjectName("Cpu_4")
        self.line = QtWidgets.QFrame(self.centralwidget)
        self.line.setGeometry(QtCore.QRect(-10, 200, 791, 20))
        
        self.line.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line.setLineWidth(3)
        self.line.setFrameShape(QtWidgets.QFrame.HLine)
        self.line.setObjectName("line")
        
        self.Cpu_Bar = QtWidgets.QProgressBar(self.centralwidget)
        self.Cpu_Bar.setGeometry(QtCore.QRect(250, 170, 521, 23))
        self.Cpu_Bar.setProperty("value", 24)
        self.Cpu_Bar.setFormat("")
        self.Cpu_Bar.setStyleSheet(bar_style)
        self.Cpu_Bar.setObjectName("Cpu_Bar")
        
        self.line_2 = QtWidgets.QFrame(self.centralwidget)
        self.line_2.setGeometry(QtCore.QRect(0, 140, 781, 20))
        self.line_2.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_2.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_2.setObjectName("line_2")
        
        self.Ram_1 = QtWidgets.QLabel(self.centralwidget)
        self.Ram_1.setGeometry(QtCore.QRect(10, 220, 781, 31))
        self.Ram_1.setFont(font)
        self.Ram_1.setStyleSheet(fontcolor)
        self.Ram_1.setText("<html><head/><body><p>Ram usd</p><p><br/></p><p><br/></p></body></html>")
        self.Ram_1.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Ram_1.setWordWrap(False)
        self.Ram_1.setObjectName("Ram_1")
        
        self.Ram_2 = QtWidgets.QLabel(self.centralwidget)
        self.Ram_2.setGeometry(QtCore.QRect(330, 220, 781, 31))
        self.Ram_2.setFont(font)
        self.Ram_2.setStyleSheet(fontcolor)
        self.Ram_2.setText("<html><head/><body><p>Ram max</p><p><br/></p><p><br/></p><p><br/></p></body></html>")
        self.Ram_2.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Ram_2.setWordWrap(False)
        self.Ram_2.setObjectName("Ram_2")
        
        self.line_3 = QtWidgets.QFrame(self.centralwidget)
        self.line_3.setGeometry(QtCore.QRect(0, 40, 781, 20))
        self.line_3.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_3.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_3.setObjectName("line_3")
        
        self.line_4 = QtWidgets.QFrame(self.centralwidget)
        self.line_4.setGeometry(QtCore.QRect(0, 250, 781, 20))
        self.line_4.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_4.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_4.setObjectName("line_4")
        
        self.Ram_3 = QtWidgets.QLabel(self.centralwidget)
        self.Ram_3.setGeometry(QtCore.QRect(10, 270, 781, 41))
        self.Ram_3.setFont(font)
        self.Ram_3.setStyleSheet(fontcolor)
        self.Ram_3.setText("<html><head/><body><p>Ram usg %</p><p><br/></p><p><br/></p><p><br/></p></body></html>")
        self.Ram_3.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Ram_3.setWordWrap(False)
        self.Ram_3.setObjectName("Ram_3")
        
        self.Ram_Bar = QtWidgets.QProgressBar(self.centralwidget)
        self.Ram_Bar.setGeometry(QtCore.QRect(250, 280, 521, 23))
        self.Ram_Bar.setProperty("value", 24)
        self.Ram_Bar.setFormat("")
        self.Ram_Bar.setStyleSheet(bar_style)
        self.Ram_Bar.setObjectName("Ram_Bar")
        
        self.line_5 = QtWidgets.QFrame(self.centralwidget)
        self.line_5.setGeometry(QtCore.QRect(0, 305, 791, 20))
        self.line_5.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_5.setLineWidth(3)
        self.line_5.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_5.setObjectName("line_5")
        
        self.Gpu_1 = QtWidgets.QLabel(self.centralwidget)
        self.Gpu_1.setGeometry(QtCore.QRect(10, 320, 1081, 41))
        self.Gpu_1.setFont(font)
        self.Gpu_1.setStyleSheet(fontcolor)
        self.Gpu_1.setText("<html><head/><body><p>GPU const1<br/></p></body></html>")
        self.Gpu_1.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Gpu_1.setWordWrap(False)
        self.Gpu_1.setObjectName("Gpu_1")
        
        self.line_6 = QtWidgets.QFrame(self.centralwidget)
        self.line_6.setGeometry(QtCore.QRect(0, 350, 781, 20))
        self.line_6.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_6.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_6.setObjectName("line_6")
        
        self.Gpu_2 = QtWidgets.QLabel(self.centralwidget)
        self.Gpu_2.setGeometry(QtCore.QRect(10, 370, 1081, 81))
        self.Gpu_2.setFont(font)
        self.Gpu_2.setStyleSheet(fontcolor)
        self.Gpu_2.setText("<html><head/><body><p>Gpu temp </p><p>Gpu PWR</p><p><br/></p></body></html>")
        self.Gpu_2.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Gpu_2.setWordWrap(False)
        self.Gpu_2.setObjectName("Gpu_2")
        
        self.Gpu_3 = QtWidgets.QLabel(self.centralwidget)
        self.Gpu_3.setGeometry(QtCore.QRect(330, 370, 781, 81))
        self.Gpu_3.setFont(font)
        self.Gpu_3.setStyleSheet(fontcolor)
        self.Gpu_3.setText("<html><head/><body><p>Gpu Mem1</p><p>Gpu Mem2</p><p><br/></p></body></html>")
        self.Gpu_3.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Gpu_3.setWordWrap(False)
        self.Gpu_3.setObjectName("Gpu_3")
        
        self.line_7 = QtWidgets.QFrame(self.centralwidget)
        self.line_7.setGeometry(QtCore.QRect(0, 450, 781, 20))
        self.line_7.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_7.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_7.setObjectName("line_7")
        
        self.Gpu_4 = QtWidgets.QLabel(self.centralwidget)
        self.Gpu_4.setGeometry(QtCore.QRect(10, 470, 1081, 41))
        self.Gpu_4.setFont(font)
        self.Gpu_4.setStyleSheet(fontcolor)
        self.Gpu_4.setText("<html><head/><body><p>GPU const1<br/></p></body></html>")
        self.Gpu_4.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Gpu_4.setWordWrap(False)
        self.Gpu_4.setObjectName("Gpu_4")
        
        self.Gpu_Bar = QtWidgets.QProgressBar(self.centralwidget)
        self.Gpu_Bar.setGeometry(QtCore.QRect(250, 480, 521, 23))
        self.Gpu_Bar.setProperty("value", 24)
        self.Gpu_Bar.setFormat("")
        self.Gpu_Bar.setStyleSheet(bar_style)
        self.Gpu_Bar.setObjectName("Gpu_Bar")
        
        self.line_8 = QtWidgets.QFrame(self.centralwidget)
        self.line_8.setGeometry(QtCore.QRect(0, 510, 791, 20))
        self.line_8.setFrameShadow(QtWidgets.QFrame.Plain)
        self.line_8.setLineWidth(3)
        self.line_8.setFrameShape(QtWidgets.QFrame.HLine)
        self.line_8.setObjectName("line_8")
        
        self.Drive = QtWidgets.QLabel(self.centralwidget)
        self.Drive.setGeometry(QtCore.QRect(10, 530, 1081, 41))
        self.Drive.setFont(font)
        self.Drive.setStyleSheet(fontcolor)
        self.Drive.setText("<html><head/><body><p>Drive<br/></p></body></html>")
        self.Drive.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Drive.setWordWrap(False)
        self.Drive.setObjectName("Drive")
        
        self.Net = QtWidgets.QLabel(self.centralwidget)
        self.Net.setGeometry(QtCore.QRect(330, 530, 1081, 41))
        self.Net.setFont(font)
        self.Net.setStyleSheet(fontcolor)
        self.Net.setText("<html><head/><body><p>Net<br/></p></body></html>")
        self.Net.setAlignment(QtCore.Qt.AlignLeading|QtCore.Qt.AlignLeft|QtCore.Qt.AlignTop)
        self.Net.setWordWrap(False)
        self.Net.setObjectName("Net")
        
        self.Cpu_4.raise_()
        self.Cpu_2.raise_()
        self.Cpu_3.raise_()
        self.Cpu_1.raise_()
        self.line.raise_()
        self.Cpu_Bar.raise_()
        self.line_2.raise_()
        self.Ram_1.raise_()
        self.Ram_2.raise_()
        self.line_3.raise_()
        self.line_4.raise_()
        self.Ram_3.raise_()
        self.Ram_Bar.raise_()
        self.line_5.raise_()
        self.Gpu_1.raise_()
        self.line_6.raise_()
        self.Gpu_2.raise_()
        self.Gpu_3.raise_()
        self.line_7.raise_()
        self.Gpu_4.raise_()
        self.Gpu_Bar.raise_()
        self.line_8.raise_()
        self.Drive.raise_()
        self.Net.raise_()
        
        MainWindow.setCentralWidget(self.centralwidget)
        self.statusbar = QtWidgets.QStatusBar(MainWindow)
        self.statusbar.setObjectName("statusbar")
        MainWindow.setStatusBar(self.statusbar)

        self.retranslateUi(MainWindow)
        QtCore.QMetaObject.connectSlotsByName(MainWindow)
        
        #set basic data
        self.first_set()
        
        # creating a timer object
        timer = QTimer(self.centralwidget)
        # adding action to timer
        timer.timeout.connect(self.update)
        # update the timer every second
        timer.start(1000)

    def retranslateUi(self, MainWindow):
        _translate = QtCore.QCoreApplication.translate
        MainWindow.setWindowTitle(_translate("MainWindow", "MainWindow"))
        bg = config['app']['background']
        MainWindow.setStyleSheet(f"background-color: {bg};")

    def first_set(self):
        Cpu_name = str(cpuinfo.get_cpu_info()['brand_raw'])
        self.Cpu_1.setText(f"<html><head/><body><p>{Cpu_name}<br/></p></body></html>")
        Cpu_const = processes.cpu_const()
        self.Cpu_2.setText(f"<html><head/><body><p>CPU Cores: {str(Cpu_const[0])}</p><p>CPU Threads: {str(Cpu_const[1])}</p><p><br/></p></body></html>")
        
    def update(self):
        #CPU
        Cpu_usg = processes.cpu_monit()
        self.Cpu_3.setText(f"<html><head/><body><p>CPU temp: {str(Cpu_usg[1])}°C</p><p>CPU freq: {str(Cpu_usg[2])}</p><p><br/></p></body></html>")
        self.Cpu_4.setText(f"<html><head/><body><p>CPU usage: {str(Cpu_usg[0])}%</p><p><br/></p></body></html>")
        self.Cpu_Bar.setValue(int(Cpu_usg[0]))
        
        #RAM
        Ram_usg = processes.ram_monit()
        self.Ram_1.setText(f"<html><head/><body><p>RAM total: {str(Ram_usg[0])}</p></body></html>")
        self.Ram_2.setText(f"<html><head/><body><p>RAM used: {str(Ram_usg[1])}</p></body></html>")
        self.Ram_3.setText(f"<html><head/><body><p>RAM usage: {str(Ram_usg[2])}%</p></body></html>")
        self.Ram_Bar.setValue(int(Ram_usg[2]))
        
        #GPU
        if gpuconf == 'nvidia':
            Gpu_usg = processes.nvd_gpuusg()
            Gpu_mem = processes.nvd_gpumem()
            self.Gpu_1.setText(f"<html><head/><body><p>{Gpu_usg[0]}<br/></p></body></html>")
            self.Gpu_2.setText(f"<html><head/><body><p>Gpu temp : {Gpu_usg[2]}°C</p><p>Gpu power : {Gpu_usg[3]}W</p><p><br/></p></body></html>")
            self.Gpu_3.setText(f"<html><head/><body><p>Gpu mem total : {Gpu_mem[1]}MB</p><p>Gpu mem used : {Gpu_mem[0]}MB</p><p><br/></p></body></html>")
            self.Gpu_4.setText(f"<html><head/><body><p>Gpu usage : {Gpu_usg[1]}%</p><p><br/></p></body></html>")
            self.Gpu_Bar.setValue(int(Gpu_usg[1]))
        elif gpuconf == 'amd':
            pass
        elif gpuconf == 'intel':
            self.Gpu_1.setText(f"<html><head/><body><p>Intel not supported<br/></p></body></html>")
        
        #Drive and Net
        try:
            Drive = processes.drives_monit_linux()
        except:
            Drive = processes.drives_monit_win()
        Net = processes.network_usage()
        self.Drive.setText(f"<html><head/><body><p>Main drive usage: {Drive} %<br/></p></body></html>")
        self.Net.setText(f"<html><head/><body><p>Network download: {round(Net, 2)} Mb/s%<br/></p></body></html>")

if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    MainWindow = QtWidgets.QMainWindow()
    ui = Ui_MainWindow()
    ui.setupUi(MainWindow)
    MainWindow.show()
    sys.exit(app.exec_())
