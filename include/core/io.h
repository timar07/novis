#ifndef NV_IO_H
#define NV_IO_H

#include <stddef.h>

#define MAX_FNAME_LEN 255

typedef struct {
    char *raw; // input as a string
    size_t len; // input length
    char name[MAX_FNAME_LEN]; // stream name
} NovisInput;

// Read source from a file by it's name
NovisInput *
novis_read(const char *fname);

#endif
