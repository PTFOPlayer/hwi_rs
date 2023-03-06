# making sure everything is up to date
git fetch & git pull

#build rust part with cargo
cargo build --release

#build msr_server (https://github.com/PTFOPlayer/msr_server) 
cd ./msr_server
./build.sh
CONFDIR= ls ~/.config | grep 'hwi_rs'
if !(grep -q "$CONFDIR" <<< "hwi_rs");
then
    echo "exists"
    mkdir ~/.config/hwi_rs
fi

cd ..
cp ./src/settings.toml ~/.config/hwi_rs/ 
#
#cd ..
#sudo cp ./target/release/hwi_rs /usr/bin
