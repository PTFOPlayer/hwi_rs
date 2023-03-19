VARDIR= ls /var/ | grep 'msr_server'
if !(grep -q "$VARDIR" <<< msr_server)
then
    sudo mkdir /var/msr_server
fi

CONFDIR= ls ~/.config | grep 'hwi_rs'
if !(grep -q "$CONFDIR" <<< "hwi_rs");
then
    mkdir ~/.config/hwi_rs
fi

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