
{
    sudo apt install python3 python3-pip --noconfirm
} || {
    sudo pacman -S python3 python3-pip --noconfirm
} || {
    sudo dnf install python3 python3-pip 
}

pip install pyqt5 psutil py-cpuinfo

