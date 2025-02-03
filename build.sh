cd tests
DEV_MAPPINGS=1 cargo build
# cargo build
cd ..
cp ./tests/target/debug/librustcraft_test.so ./build/out/
