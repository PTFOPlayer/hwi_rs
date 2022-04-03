import tkinter as tk
from tkinter.ttk import *
import processes
import json
import os

with open('config.json') as configf:
        #read json file
        config = json.load(configf)
        #print data
        config.get('gpu').get('show')

root = tk.Tk()
root.title("PC Stats")
root.configure(background='black', padx=20)
root.attributes("-alpha", 0.85)


style1 = Style()
style1.theme_use('clam')
style1.configure("red.Horizontal.TProgressbar", foreground='red', background='red')

filler1_1 = tk.Frame(root, bg = 'grey', width = 400, height = 3)
filler1_2 = tk.Frame(root, bg = 'grey', width = 400, height = 3)
filler2_1 = tk.Frame(root, bg = 'grey', width = 400, height = 3)
filler2_2 = tk.Frame(root, bg = 'grey', width = 400, height = 3)
filler3_1 = tk.Frame(root, bg = 'grey', width = 400, height = 3)
filler3_2 = tk.Frame(root, bg = 'grey', width = 400, height = 3)


cpu_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
cpu_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
ram_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
ram_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
gpu_monit = tk.Label(root , foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
gpu_graph = Progressbar(root, style="red.Horizontal.TProgressbar", orient="horizontal", length=400, mode='determinate')
drives_monit = tk.Label(root, foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))
net_monit = tk.Label(root, foreground = 'white', background = 'black', font = ('Helvetica', 12, 'bold'))

cpu_monit.grid(row=0, column=0)
cpu_graph.grid(row=0, column=1)
filler1_1.grid(row=1, column=0)
filler1_2.grid(row=1, column=1)

ram_monit.grid(row=2, column=0)
ram_graph.grid(row=2, column=1)
filler2_1.grid(row=3, column=0)
filler2_2.grid(row=3, column=1)

gpu_monit.grid(row=4, column=0)
gpu_graph.grid(row=4, column=1)
filler3_1.grid(row=5, column=0)
filler3_2.grid(row=5, column=1)

drives_monit.grid(row=6, column=0)
net_monit.grid(row=6, column=1)

def update_monit():
    if config.get('cpu').get('show'):
        temporary = processes.cpu_monit()[0]
        
        cpu_monit.config(text=" \n CPU usage in % : "
                        + str(temporary) + "\n"
                        + "CPU cores : "  + str(processes.cpu_monit()[1]) + "\n"
                        + "CPU threads : " + str(processes.cpu_monit()[2]) + "\n"
                        + "CPU temperature in °C : "+str(processes.cpu_monit()[3]) + "\n")
        cpu_graph['value'] = temporary 
    elif config.get('cpu').get('show') == False:
        cpu_monit.destroy()
        cpu_graph.destroy()
        filler1_1.destroy()
        filler1_2.destroy()
        
        
    if config.get('ram').get('show'):
        if config.get('ram').get('type') == 'full':
            ram_monit.config(text="\n RAM usage in GB : \n" 
                            + "Total : " + str(processes.ram_monit()[0]) + "\n"
                            + "Used : " + str(processes.ram_monit()[1]) + "\n"
                            + "RAM usage in % : " + str(processes.ram_monit()[2]) + "\n")
            ram_graph['value'] = processes.ram_monit()[2]
        elif config.get('ram').get('type') == 'simple':
            ram_monit.config(text="RAM usage in % : " + str(processes.ram_monit()[2]))
            ram_graph['value'] = processes.ram_monit()[2]
    elif config.get('ram').get('show') == False:
        ram_monit.destroy()
        ram_graph.destroy()
        filler2_1.destroy()
        filler2_2.destroy()
        
        
    try:
        if config.get('gpu').get('show') == True:
            gpu_usg = processes.nvd_gpuusg()
            gpu_mem = processes.nvd_gpumem()
            if config.get('gpu').get('type') == 'full':
                gpu_monit.config(text= "\n GPU model : " + str(gpu_usg[0]) + "\n"
                                + "GPU mem usage : " + "\n"
                                + "Total : " + str(gpu_mem[1]) + " MB     " 
                                + "Used : " + str(gpu_mem[0]) + " MB \n"
                                + "GPU temperature in : " + str(gpu_usg[2]) + " °C \n"
                                + "GPU power : " + str(gpu_usg[3]) + " \n"
                                + "\n GPU usage : " + str(gpu_usg[1]) + "% \n")
                gpu_graph['value'] = gpu_usg[1]
            elif config.get('gpu').get('type') == 'simple':
                gpu_monit.config(text= "GPU model : " + str(gpu_usg[0]) + "\n" 
                                 + "GPU usage : " + str(gpu_usg[1]) + "%")
                gpu_graph['value'] = gpu_usg[1]
        elif config.get('gpu').get('show') == False:
            gpu_monit.destroy()
            gpu_graph.destroy()
            filler3_1.destroy()
            filler3_2.destroy()
            
    except:
        if config.get('gpu').get('show') == True:
            gpu_monit.config(text="No compatible GPU detected")
            gpu_graph['value'] = 0
        elif config.get('gpu').get('show') == False:
            gpu_monit.destroy()
            gpu_graph.destroy()
            filler3_1.destroy()
            filler3_2.destroy()
            
            
    try:
        drives_monit.config(text= "\n C drive usage in % : " + str(processes.drives_monit_win()) + "\n")
    except:
        drives_monit.config(text= "\n sda drive usage in % : " + str(processes.drives_monit_linux()) + "\n")
    net_monit.config(text = "\n Network transfer in Mb/s : " + str(processes.network_usage()) + "\n")
    root.after(800, update_monit)
    
    
update_monit()
root.mainloop()
