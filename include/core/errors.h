#ifndef NOVIS_ERRORS_H
#define NOVIS_ERRORS_H

#include <stddef.h>

void novis_error(const char *fname, size_t line, size_t col,
                 const char *errtype, const char *msg);

void novis_fatal(const char *fname, size_t line, size_t col,
                                            const char *msg);

#endif
