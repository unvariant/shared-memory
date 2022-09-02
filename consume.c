#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <inttypes.h>
#include <string.h>

int main () {
    int fd = shm_open ("/test", O_RDWR, 0);
    printf ("fd: %d\n", fd);

    if (fd < 0) {
        perror ("unable to open shared memory");
        exit (1);
    }

    int rs = ftruncate (fd, 2048);
    printf ("ftruncate: %d\n", rs);

    char * buf = mmap (0, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    printf ("buffer address: %p\n", buf);

    char msg[] = "do not share memory by communicating, communicate by sharing memory.";
    printf("writing data to buffer\n");

    strcpy (buf+1, msg);

    *buf = 1;

    close (fd);

    return 0;
}