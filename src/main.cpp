#include <iostream>
#include <string>
#include "bind.h"


int main()
{
    auto [x, y, z, w] = get_request((char*)"https://example.com");
	auto [x1, y1, z1, w1] = simulate_data_request();
    std::printf("data 1:\n\tx: %hhi\n\ty: %u\n\tz: %li\n\tw: %s\n\n", x, y, z, w);
    std::printf("data 2:\n\tx: %hhi\n\ty: %u\n\tz: %li\n\tw: %s\n\n", x1, y1, z1, w1);
    return 0;
}
