cd msr_server
./build.sh
cd ..

cargo tauri build --verbose
pdir=./install_deps
cp src-tauri/target/release/bundle/appimage/hwi-rs*amd64.AppImage $pdir
cd $pdir
split -b95M hwi-rs*amd64.AppImage bin_part.
cd ..

cp msr_server/msr_server.service $pdir
cp msr_server/target/release/libmsr_rs.so $pdir
cp msr_server/msr_gen $pdir
cp hwi_rs.desktop $pdir