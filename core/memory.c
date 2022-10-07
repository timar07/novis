#include <stdlib.h>

void *
novis_alloc(size_t size)
{
    return malloc(size);
}

void
novis_dealloc(void *mem)
{
    return free(mem);
}