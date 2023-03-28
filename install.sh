sudo mkdir /var/msr_server
sudo mkdir /var/hwi_rs
sudo touch /var/hwi_rs/radeon
mkdir ~/.config/hwi_rs


if [ "$1" = "-s" ];
then 
    if (sudo cp ./instalation_files/msr_server.ts /var/msr_server/) && (sudo cp ./instalation_files/package.json /var/msr_server) && (sudo npm i --prefix /var/msr_server) 
    then
        echo "installed additional dependencies"
    else
        echo "error building"
    fi
fi

if (sudo cp ./instalation_files/msr_server.service /etc/systemd/system/) && (sudo cp ./instalation_files/msr_rest_server.service /etc/systemd/system/) && (sudo cp ./instalation_files/msr_gen /usr/bin/) && (cp ./instalation_files/settings.toml ~/.config/hwi_rs/) && (sudo cp ./instalation_files/hwi_rs /usr/bin)
then
    echo "succes, build ended"
else 
    echo "error building"
fi