#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct data {
  uint8_t number_char;
  uint32_t number_int;
  int64_t number_long;
  const char *string;
};

extern "C" {

extern void write(uint64_t x, const char *y, uint64_t z);

data get_request(char *http);

data simulate_data_request();

} // extern "C"
