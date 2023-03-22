sudo mkdir /var/msr_server
mkdir ~/.config/hwi_rs

if (sudo cp ./instalation_files/msr_server.service /etc/systemd/system/) && (sudo cp ./instalation_files/msr_rest_server.service /etc/systemd/system/) && (sudo cp ./instalation_files/msr_gen /usr/bin/) && (cp ./instalation_files/settings.toml ~/.config/hwi_rs/) && (sudo cp ./instalation_files/hwi_rs /usr/bin)
then
    if (sudo cp ./instalation_files/msr_server.ts /var/msr_server/) && (sudo cp ./instalation_files/package.json /var/msr_server) && (sudo npm i --prefix /var/msr_server)
    then 
        echo "succes, build ended"
    else
        echo "error building"
    fi
else 
    echo "error building"
fi