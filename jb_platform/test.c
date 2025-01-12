#include <time.h>
#include <errno.h>
#include <string.h>

int main() {
    time_t current_time = time(nullptr);
    tm loctime;
    localtime_s(&loctime, &current_time);
    int i = EINVAL;

    char buf[256];
    size_t siz = strftime(buf, 256, "", &loctime);
}