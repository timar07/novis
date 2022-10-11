#ifndef NV_LEXER_H
#define NV_LEXER_H

#include "core/io.h"
#include <stddef.h>

typedef enum {
    // One character long
    TOKEN_PLUS = 0,
    TOKEN_MINUS,
    TOKEN_STAR,
    TOKEN_SLASH,
    TOKEN_DOT,
    TOKEN_COMMA,
    TOKEN_LESS,
    TOKEN_GREATER,
    TOKEN_EQUAL,
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    // Two character long
    TOKEN_ARROW_LEFT,
    TOKEN_ARROW_RIGHT,
    TOKEN_EQUAL_EQUAL,
    TOKEN_BANG_EQUAL,
    TOKEN_LESS_EQUAL,
    TOKEN_GREATER_EQUAL,
    // Keywords
    TOKEN_FALSE,
    TOKEN_TRUE,
    TOKEN_FUNC,
    // Other
    TOKEN_NUMBER,
    TOKEN_STRING,
    TOKEN_IDENTIFIER,
    TOKEN_ERROR,
    TOKEN_EOF
} NovisTokenTag;

#ifdef NV_DEBUG_LEXER
static const char * _novis_tags_strings[] =  {
    "PLUS",
    "MINUS",
    "STAR",
    "SLASH",
    "DOT",
    "COMMA",
    "LESS",
    "GREATER",
    "EQUAL",
    "LPAREN",
    "RPAREN",
    "ARROW_LEFT",
    "ARROW_RIGHT",
    "EQUAL_EQUAL",
    "BANG_EQUAL",
    "LESS_EQUAL",
    "GREATER_EQUAL",
    "FALSE",
    "TRUE",
    "FUNC",
    "NUMBER",
    "STRING",
    "IDENTIFIER",
    "ERROR",
    "<EOF>"
};

const char *
_novis_get_tag_string(NovisTokenTag tag);
#endif

typedef struct {
    size_t      current; // current lexing symbol
    size_t      start;   // start of the current token
    size_t      end;     // end of the current token
    size_t      line;    // current line
    NovisInput *src;     // lexer source string
} NovisLexer;

typedef struct {
    NovisTokenTag tag;     // token type
    NovisLexer    ls;      // lexical state
    const char    *lexeme;  // lexeme (substring from source file)
} NovisToken;

void
novis_init_lexer(NovisLexer *lexer, NovisInput *src);

NovisToken
novis_lex(NovisLexer *self);

#endif
