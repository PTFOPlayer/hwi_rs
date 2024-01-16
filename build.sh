cd msr_server
./build.sh
cd ..
cargo build --release
pdir=./install_deps
cp target/release/hwi_rs $pdir
cp msr_server/msr_server.service $pdir
cp msr_server/msr_gen $pdir
cp hwi_rs.desktop $pdir