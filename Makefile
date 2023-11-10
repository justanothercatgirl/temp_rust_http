
RUST_FILES = src/lib.rs
RUSTC = cargo
HEADERGEN = cbindgen
CXXFLAGS = -std=c++2a 
LDFLAGS = -Ltarget/release
LDLIBS = -lhttp_test
CXX = g++
BUILD_DIR = target/release
VPATH = src:$(BUILD_DIR)


test: libhttp_test.so bind.h main.o
	$(CXX) -o $(BUILD_DIR)/$@ $(BUILD_DIR)/main.o -Wl,-rpath,./

libhttp_test.so: lib.rs
	cargo build --release

bind.h: lib.rs
	$(HEADERGEN) --lang c++ --output /src/$@

main.o: main.cpp bind.h
	$(CXX) -c -o $(BUILD_DIR)/main.o $<


.PHONY: clean
clean:



