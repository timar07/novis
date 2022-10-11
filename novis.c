#include "compiler/parser.h"
#include "compiler/lexer.h"
#include "compiler/debug_macros.h"
#include <stdio.h>

int main(int argc, char *argv[])
{
    NovisLexer lexer;
    novis_init_lexer(&lexer, novis_read(argv[1]));
    novis_parse(&lexer);

    return 0;
}