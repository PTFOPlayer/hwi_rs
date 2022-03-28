import tkinter as tk
from tkinter.ttk import *
import processes
import psutil
from time import sleep

root = tk.Tk()
s = Style()
s.theme_use('clam')
s.configure("red.Horizontal.TProgressbar", foreground='red', background='red')
root.configure(background='black')
cpu_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
cpu_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
ram_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
ram_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
gpu_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
gpu_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
drives_monit = tk.Label(root, foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
net_monit = tk.Label(root, foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
cpu_monit.grid(row=0, column=0)
cpu_graph.grid(row=1, column=0)
ram_monit.grid(row=2, column=0)
ram_graph.grid(row=3, column=0)
gpu_monit.grid(row=4, column=0)
gpu_graph.grid(row=5, column=0)
drives_monit.grid(row=7, column=0)
net_monit.grid(row=8, column=0)
root.title("PC Stats")

def update_monit():
    temporary = processes.cpu_monit()[0]
    cpu_monit.config(text="CPU usage in % : "
                      + str(temporary) + "\n"
                      + "CPU temperature in °C : "+str(processes.cpu_monit()[1]))
    cpu_graph['value'] = temporary 
    ram_monit.config(text="RAM usage in GB : \n" 
                     + "Total : " + str(processes.ram_monit()[0]) + "\n"
                     + "Used : " + str(processes.ram_monit()[1]) + "\n"
                     + "RAM usage in % : " + str(processes.ram_monit()[2]))
    ram_graph['value'] = processes.ram_monit()[2]
    try:
        gpu_monit.config(text= "GPU model : " + str(processes.nvd_gpuusg()[0])
                        + "GPU mem usage in MB : " + "\n"
                        + "Total GPU memory : " + str(processes.nvd_gpumem()[1]) + " MB \n"
                        + "Used GPU memory : " + str(processes.nvd_gpumem()[0]) + " MB \n"
                        + "GPU temperature in °C : " + str(processes.nvd_gpuusg()[2]) + "\n"
                        + "GPU power in W : " + str(processes.nvd_gpuusg()[3])
                        + "GPU usage in % : " + str(processes.nvd_gpuusg()[1]))
        gpu_graph['value'] = processes.nvd_gpuusg()[1]
    except:
        gpu_monit.config(text="No compatible GPU detected")
        gpu_graph['value'] = 0
    try:
        drives_monit.config(text= " \n C drive usage in % : " + str(processes.drives_monit_win()[0]) + str(processes.drives_graph_win()[0]) + "\n" +
                            "D drive usage in % : " + str(processes.drives_monit_win()[1]) + str(processes.drives_graph_win()[1]) +"\n" +
                            "E drive usage in % : " + str(processes.drives_monit_win()[2]) + str(processes.drives_graph_win()[2]) + "\n" +
                            "F drive usage in % : " + str(processes.drives_monit_win()[3]) + str(processes.drives_graph_win()[3]) + "\n" +
                            "G drive usage in % : " + str(processes.drives_monit_win()[4]) + str(processes.drives_graph_win()[4]))
    except:
        drives_monit.config(text= "sda drive usage in % : " + str(processes.drives_monit_linux()[0]) + str(processes.drives_graph_linux()[0]))
                            #+ "\n" +
                            #"sdb drive usage in % : " + str(processes.drives_monit_linux()[1]) + str(processes.drives_graph_linux()[1]) +"\n" +
                            #"sdc drive usage in % : " + str(processes.drives_monit_linux()[2]) + str(processes.drives_graph_linux()[2]) + "\n" +
                            #"sdd drive usage in % : " + str(processes.drives_monit_linux()[3]) + str(processes.drives_graph_linux()[3]) + "\n" +
                            #"sde drive usage in % : " + str(processes.drives_monit_linux()[4]) + str(processes.drives_graph_linux()[4]))
                            #linux drives under work
    net_monit.config(text = "Network usage in Mb : " + str(processes.network_usage()))
    root.after(250, update_monit)
update_monit()
root.mainloop()
