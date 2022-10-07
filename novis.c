#include "compiler/lexer.h"
#include "compiler/debug_macros.h"
#include <stdio.h>

int main(int argc, char *argv[])
{
    novis_lex(novis_read(argv[1]));
    return 0;
}