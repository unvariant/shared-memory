#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <inttypes.h>
#include "atomic.c"

int main () {
    int fd = shm_open ("/test", O_RDWR);
    printf ("fd: %d\n", fd);

    if (fd < 0) {
        perror ("unable to open shared memory");
        exit (1);
    }

    int rs = ftruncate (fd, 2048);
    printf ("result: %d\n", rs);

    uint64_t * buf = mmap (0, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    printf ("buf: %p\n", buf);

    atomic_store64 (buf, 0x100);

    close (fd);

    return 0;
}