import psutil
import os
def cpu_monit():
    #monitoring CPU usage
    cpu_usg = psutil.cpu_percent()
    #monitoring CPU temperature
    try:
        temp = psutil.sensors_temperatures()['coretemp'][0].current
    except:
        temp = "not accessible"
    return cpu_usg, temp


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
    command = "nvidia-smi --query-gpu=memory.used --format=csv,noheader,nounits"
    gpmem_usg = int(os.popen(command).read())
    #total GPU memory
    command = "nvidia-smi --query-gpu=memory.total --format=csv,noheader,nounits"
    gpmem_max = int(os.popen(command).read())

    return gpmem_usg, gpmem_max

def nvd_gpuusg():
    #monitoring Nvidia GPU usage
    command = "nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits & nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits & nvidia-smi --query-gpu=name --format=csv,noheader,nounits & nvidia-smi --query-gpu=power.draw --format=csv,noheader,nounits"
    gpu_usg = os.popen(command).read()
    split_gpu_usg = gpu_usg.split('\n')
    gpu_usg = split_gpu_usg[0]
    gpu_temp = split_gpu_usg[1]
    gpu_name = split_gpu_usg[2]
    gpu_power = split_gpu_usg[3]
    return gpu_name, gpu_usg, gpu_temp , gpu_power

def radeonn_gpu_usg():
    #monitoring Radeon GPU usage
    print("\n    AMD GPU usage not supported at the moment")
