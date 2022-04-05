import psutil
import os

from psutil import cpu_freq

def cpu_const():
    #monitoring CPU cores
    cpu_cores = psutil.cpu_count(logical=False)
    #monitoring CPU threads
    cpu_threads = psutil.cpu_count(logical=True)
    return cpu_cores, cpu_threads

def cpu_monit():
    #monitoring CPU usage
    cpu_usg = psutil.cpu_percent()
    #monitoring CPU temperature
    try:
        temp = psutil.sensors_temperatures()['coretemp'][0].current
    except:
        temp = "not accessible"
    #monitoring CPU frequency
    cpu_freq = round(psutil.cpu_freq().current, 3)
    return cpu_usg, temp, cpu_freq

def ram_monit():
    #monitoring RAM usage
    ram_max = round(psutil.virtual_memory().total / 1024 / 1024 / 1024, 2)
    ram_usd = round(psutil.virtual_memory().used / 1024 / 1024 / 1024 , 2)
    ram_usg = psutil.virtual_memory().percent
    return ram_max, ram_usd, ram_usg

def drives_monit_win():
    #monitoring disk usage on windows
    cdrive_usg = psutil.disk_usage('C:\\').percent
    return cdrive_usg

def drives_monit_linux():
    #monitoring disk usage on linux
    try:
        sdadrive_usg = psutil.disk_usage('/').percent
    except:
        sdadrive_usg = 0
    return sdadrive_usg

net_usg = [1 , 2]
def network_usage():
    #monitoring network usage 
    net_usg[1] = psutil.net_io_counters().bytes_recv
    net_out = net_usg[1] - net_usg[0]
    net_rec = round(net_out / 1024 / 1024 * 8, 4)
    net_usg[0] = net_usg[1]
    return net_rec

def nvd_gpumem():
    #monitoring Nvidia GPU memory usage
    command = "nvidia-smi --query-gpu=memory.used,memory.total --format=csv,noheader,nounits"
    gpu_mem = os.popen(command).read()
    sep_gpu_mem = gpu_mem.split(',')
    gpmem_usg = round(int(sep_gpu_mem[0]), 2)
    gpmem_max = round(int(sep_gpu_mem[1]), 2)

    return gpmem_usg, gpmem_max

def nvd_gpuusg():
    #monitoring Nvidia GPU usage
    command = "nvidia-smi --query-gpu=utilization.gpu,temperature.gpu,name,power.draw --format=csv,noheader,nounits"
    gpu_usg = os.popen(command).read()
    split_gpu_usg = gpu_usg.split(',')
    gpu_usg = split_gpu_usg[0]
    gpu_temp = split_gpu_usg[1]
    gpu_name = split_gpu_usg[2]
    gpu_power = split_gpu_usg[3]
    
    return gpu_name, gpu_usg, gpu_temp, gpu_power


def amd_gpuusg():
    #monitoring Radeon GPU usage
    command = 'radeontop -l 1 -d -'
    amd_gpu_usg = os.popen(command).read()
    amd_gpu_usg = amd_gpu_usg.split(',')
    #print AMD gpu usage, splitted by line

    gpu_usg_split = amd_gpu_usg[2].split(' ')
    gpu_util_split = gpu_usg_split[2].split('%')
    gpu_util = float(gpu_util_split[0])
    gpu_mem = amd_gpu_usg[13]
    return gpu_util, gpu_mem