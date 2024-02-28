cp -rfv ~/storage/shared/CODED/* ~/CODED/
cargo run
if [ $? -eq 130 ]; then
    mv dictionary ~/storage/shared/
fi
