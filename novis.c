#include "compiler/lexer.h"
#include "compiler/debug_macros.h"
#include <stdio.h>

int main(int argc, char *argv[])
{
    novis_lexer_t lexer;
    novis_init_lexer(&lexer, novis_read(argv[1]));

    novis_token_t *token = novis_lex(&lexer);

    while (token->toktype != TOKEN_EOF)
        token = novis_lex(&lexer);

    return 0;
}