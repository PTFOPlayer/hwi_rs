cat ./install_deps/bin_part.* > ./install_deps/hwi_rs
chmod +x ./install_deps/hwi_rs

sudo cp ./install_deps/*.service /etc/systemd/system/
sudo cp ./install_deps/msr_gen /usr/bin/
sudo cp ./install_deps/hwi_rs /usr/bin
sudo cp ./install_deps/hwi_rs.desktop /usr/share/applications/
sudo cp ./install_deps/libmsr_rs.so /usr/lib/
sudo LD_LIBRARY_PATH=/usr/lib/libmsr_rs.so:$LD_LIBRARY_PATH bash -c 'echo $LD_LIBRARY_PATH'


