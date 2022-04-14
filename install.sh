#segment to install the program on pc
bash requirements.sh
_pwd=$(pwd)
echo  "cd $_pwd && python3 app.py" > PYHW
bash -c "chmod +x PYHW"

sudo cp PYHW /bin
sudo cp PYHW.desktop ~/.local/share/applications/
