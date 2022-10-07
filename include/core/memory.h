#ifndef NV_MEMORY_H
#define NV_MEMORY_H

#include <stddef.h>

void *
novis_alloc(size_t size);

void
novis_dealloc(void *mem);

#endif
