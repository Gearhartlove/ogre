
#!/usr/bin/env sh
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/ --target web ./target/wasm32-unknown-unknown/release/ogre.wasm
# get current date
now=$(date +'%m%d%Y')
rm -r ~/Desktop/ogre_builds/bevybuild$now

mkdir ~/Desktop/ogre_builds/bevybuild$now
cp -r ./assets ~/Desktop/ogre_builds/bevybuild$now
cp -r ./web ~/Desktop/ogre_builds/bevybuild$now
cp ./index.html ~/Desktop/ogre_builds/bevybuild$now