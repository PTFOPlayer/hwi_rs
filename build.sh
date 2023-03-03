#build rust part with cargo
cargo build --release

#building msr_server (https://github.com/PTFOPlayer/msr_server) 
cd ./msr_server
./build.sh
