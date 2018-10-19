#include <stdio.h>
#include <ctype.h>

int main(void) {
    for (int i = 0; i < 255; i++) {
        if (isascii(i) && !isblank(i) && !iscntrl(i)) 
            fprintf(stdout, "%c\n", i);
    }
    return 0;
}