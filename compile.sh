echo "building rust"
cargo build --release
echo "building header"
cbindgen --lang c++ --output src/bind.h .
echo "building c++"
g++ -std=c++20 src/main.cpp -o target/release/test -Ltarget/release -lhttp_test -Wl,-rpath,./
echo "done
