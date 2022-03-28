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
    #monitoring disks usage on windows
    cdrive_usg = psutil.disk_usage('C:\\').percent
    try:
        ddrive_usg = psutil.disk_usage('D:\\').percent
    except:
        ddrive_usg = 0
    try:
        edrive_usg = psutil.disk_usage('E:\\').percent
    except:
        edrive_usg = 0
    try:
        fdrive_usg = psutil.disk_usage('F:\\').percent
    except:
        fdrive_usg = 0
    try:
        gdrive_usg = psutil.disk_usage('G:\\').percent
    except:
        gdrive_usg = 0
    return cdrive_usg, ddrive_usg, edrive_usg, fdrive_usg, gdrive_usg

def drives_graph_win():
    #drives usage graph
    cdrive_usg = psutil.disk_usage('C:\\').percent
    try:
        ddrive_usg = psutil.disk_usage('D:\\').percent
    except:
        ddrive_usg = 0
    try:
        edrive_usg = psutil.disk_usage('E:\\').percent
    except:
        edrive_usg = 0
    try:
        fdrive_usg = psutil.disk_usage('F:\\').percent
    except:
        fdrive_usg = 0
    try:
        gdrive_usg = psutil.disk_usage('G:\\').percent
    except:
        gdrive_usg = 0
    #drive C usage graph
    cgraph = ""
    round_cdrive = int(round(cdrive_usg, 0) / 5)
    cgraph+= "|"
    for i in range(round_cdrive):
        cgraph+="'"
    for i in range(20 - round_cdrive):
        cgraph+=" "
    cgraph+="|"
    #drive D usage graph
    dgraph = ""
    dgraph+= "|"
    round_ddrive = int(round(ddrive_usg, 0) / 5)
    for i in range(round_ddrive):
        dgraph+="'"
    for i in range(20 - round_ddrive):
        dgraph+=" "
    dgraph+="|"
    #drive E usage graph
    egraph = ""
    egraph+= "|"
    round_edrive = int(round(edrive_usg, 0) / 5)
    for i in range(round_edrive):
        egraph+="'"
    for i in range(20 - round_edrive):
        egraph+=" "
    egraph+="|"
    #drive F usage graph
    fgraph = ""
    fgraph+= "|"
    round_fdrive = int(round(fdrive_usg, 0) / 5)
    for i in range(round_fdrive):
        fgraph+="'"
    for i in range(20 - round_fdrive):
        fgraph+=" "
    fgraph+="|"
    #drive G usage graph
    ggraph = ""
    ggraph+= "|"
    round_gdrive = int(round(gdrive_usg, 0) / 5)
    for i in range(round_gdrive):
        ggraph+="'"
    for i in range(20 - round_gdrive):
        ggraph+=" "
    ggraph+="|"
    return cgraph, dgraph, egraph, fgraph, ggraph

def drives_monit_linux():
    #monitoring disks usage on linux
    try:
        sdadrive_usg = psutil.disk_usage('/').percent
    except:
        sdadrive_usg = 0
    #try:
    #    sdbdrive_usg = psutil.disk_usage('/dev/sdb').percent
    #except:
    #    sdbdrive_usg = 0
    #try:
    #    sdcdrive_usg = psutil.disk_usage('/dev/sdc').percent
    #except:
    #    sdcdrive_usg = 0
    #try:
    #    sdddrive_usg = psutil.disk_usage('/dev/sdd').percent
    #except:
    #    sdddrive_usg = 0
    #try:
    #    sdedrive_usg = psutil.disk_usage('/dev/sde').percent
    #except:
    #    sdedrive_usg = 0 """
    return sdadrive_usg #, sdbdrive_usg, sdcdrive_usg, sdddrive_usg, sdedrive_usg
    #linux drives under work
def drives_graph_linux():
    #drives usage graph
    try:
        sdadrive_usg = psutil.disk_usage('/').percent
    except:
        sdadrive_usg = 0
    # try:
    #    sdbdrive_usg = psutil.disk_usage('/dev/sdb').percent
    #except:
    #    sdbdrive_usg = 0
    #try:
    #    sdcdrive_usg = psutil.disk_usage('/dev/sdc').percent
    #except:
    #    sdcdrive_usg = 0
    #try:
    #    sdddrive_usg = psutil.disk_usage('/dev/sdd').percent
    #except:
    #    sdddrive_usg = 0
    #try:
    #    sdedrive_usg = psutil.disk_usage('/dev/sde').percent
    #except:
    #    sdedrive_usg = 0
    #drive sda usage graph
    sda_graph = ""
    sda_graph+= "|"
    round_sdadrive = int(round(sdadrive_usg, 0) / 5)
    for i in range(round_sdadrive):
        sda_graph+="'"
    for i in range(20 - round_sdadrive):
        sda_graph+=" "
    sda_graph+="|"
    #
    ##drive sdb usage graph
    #sdb_graph = ""
    #sdb_graph+= "|"
    #round_sdbdrive = int(round(sdbdrive_usg, 0) / 5)
    #for i in range(round_sdbdrive):
    #    sdb_graph+="'"
    #for i in range(20 - round_sdbdrive):
    #    sdb_graph+=" "
    #sdb_graph+="|"
    ##drive sdc usage graph
    #sdc_graph = ""
    #sdc_graph+= "|"
    #round_sdcdrive = int(round(sdcdrive_usg, 0) / 5)
    #for i in range(round_sdcdrive):
    #    sdc_graph+="'"
    #for i in range(20 - round_sdcdrive):
    #    sdc_graph+=" "
    #sdc_graph+="|"
    ##drive sdd usage graph
    #sdd_graph = ""
    #sdd_graph+= "|"
    #round_sdddrive = int(round(sdddrive_usg, 0) / 5)
    #for i in range(round_sdddrive):
    #    sdd_graph+="'"
    #for i in range(20 - round_sdddrive):
    #    sdd_graph+=" "
    #sdd_graph+="|"
    ##drive sde usage graph
    #sde_graph = ""
    #sde_graph+= "|"
    #round_sdedrive = int(round(sdedrive_usg, 0) / 5)
    #for i in range(round_sdedrive):
    #    sde_graph+="'"
    #for i in range(20 - round_sdedrive):
    #    sde_graph+=" "
    #sde_graph+="|" 
    return sda_graph #, sdb_graph, sdc_graph, sdd_graph, sde_graph
    
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
    command = "nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits"
    gpu_usg = int(os.popen(command).read())
    #monitoring GPU temperature
    command = "nvidia-smi --query-gpu=temperature.gpu --format=csv,noheader,nounits"
    gpu_temp =  int(os.popen(command).read())
    #gpu name
    command = "nvidia-smi --query-gpu=name --format=csv,noheader,nounits"
    gpu_name = os.popen(command).read()
    return gpu_name, gpu_usg, gpu_temp

def radeonn_gpu_usg():
    #monitoring Radeon GPU usage
    print("\n    AMD GPU usage not supported at the moment")
