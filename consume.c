#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <inttypes.h>

int main () {
    int fd = shm_open ("/test", O_RDWR, 0);
    printf ("fd: %d\n", fd);

    if (fd < 0) {
        perror ("unable to open shared memory");
        exit (1);
    }

    int rs = ftruncate (fd, 2048);
    printf ("ftruncate: %d\n", rs);

    uint64_t * buf = mmap (0, 1024, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    printf ("buffer address: %p\n", buf);

    uint64_t n = 0x100;
    printf("writing %ld to buffer\n", n);
    atomic_store64 (buf, n);

    close (fd);

    return 0;
}