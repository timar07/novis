#include "core/errors.h"
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>

#define RED_STR(str) "\x1B[31m"str"\x1B[0m"
#define ERR_INDENT 4

static void
print_error(const char *fname, size_t line, size_t col,
            const char errtype[], const char msg[])
{
    fprintf(
        stderr,
        "%s {%lu:%lu} "RED_STR("%s:")"\n%*s%s\n",
        fname,
        line,
        col,
        errtype,
        ERR_INDENT,
        "",
        msg
    );
}

void novis_error(const char *fname, size_t line, size_t col,
                 const char *errtype, const char *msg)
{
    print_error(fname, line, col, errtype, msg);
}

void novis_fatal(const char *fname, size_t line, size_t col,
                                            const char *msg)
{
    novis_error(fname, line, col, "Fatal Error", msg);
    exit(0);
}
