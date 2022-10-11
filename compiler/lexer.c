#include "compiler/lexer.h"
#include "compiler/debug_macros.h"
#include "core/errors.h"
#include "core/memory.h"
#include <stdbool.h>
#include <ctype.h>
#include <string.h>
#include <stddef.h>
#include <stdarg.h>
#include <stdio.h>

// * - Utils -

// Check if 'c' is a legal
// identifier symbol
static bool
is_identifier(int c)
{
    return (c >= 'a' && c <= 'z') ||
           (c >= 'A' && c <= 'Z') ||
            c == '_';
}

// Check if 'c' is a legal
// number literal symbol
static bool
is_digit(int c)
{
    return c >= '0' && c <= '9';
}

// Return is lexer at the end or not
static bool
is_at_end(NovisLexer *self)
{
    return self->end >= self->src->len;
}

// Return current character
static int
current(NovisLexer *self)
{
    return self->src->raw[self->end];
}

// Return current character
// and increase self->end
static int
get_raw(NovisLexer *self)
{
    int c;

    if (!is_at_end(self)) {
        c = self->src->raw[self->end++];
        return c;
    }

    return EOF;
}

// Check if current character
// matches 'c'
static bool
match(NovisLexer *self, int c)
{
    if (current(self) == c) {
        get_raw(self);
        return true;
    }

    return false;
}

// Check if there is a word
// in current position
static bool
match_word(NovisLexer *self, const char *word)
{
    size_t len = strlen(word);
    bool is_match = memcmp(&self->src->raw[self->end], word, len) == 0;

    if (self->src->len - self->end > 0 && is_match) {
        self->end += len;
        return true;
    }

    return false;
}

#ifdef NV_DEBUG_LEXER
const char *
_novis_get_tag_string(NovisTokenTag tag)
{
    return _novis_tags_strings[tag];
}

void
_novis_token_dump(NovisToken *token)
{
    enum {  DEBUG_INDENT = 4 };

    printf("Token {\n");
    printf("%*stype   = %s;\n", DEBUG_INDENT, "", _novis_get_tag_string(token->tag));
    printf("%*slexeme = %s;\n", DEBUG_INDENT, "", token->lexeme);
    printf("%*sline   = %lu;\n", DEBUG_INDENT, "", token->ls.line);
    printf("%*sstart  = %lu;\n", DEBUG_INDENT, "", token->ls.start);
    printf("%*send    = %lu;\n", DEBUG_INDENT, "", token->ls.end);
    printf("}\n");
}
#endif

// Return substring from a current file
static char *
get_substr(NovisLexer *self, size_t start, size_t end)
{
    size_t strsz = sizeof(char)*end-start+1;
    char *substr = novis_alloc(strsz);

    memcpy(substr, &self->src->raw[start], strsz-1);
    substr[strsz] = '\0';

    return substr;
}

// Create token
static NovisToken
create_token(NovisLexer *self, NovisTokenTag tag)
{
    NovisToken token;
    token.tag = tag;
    memcpy(&token.ls, self, sizeof(NovisLexer)); // copy lexical state

    if (token.tag != TOKEN_EOF) {
        token.lexeme = get_substr(self, token.ls.start, token.ls.end);
    } else {
        token.lexeme = "<EOF>";
    }

#ifdef NV_DEBUG_LEXER
    _novis_token_dump(&token);
#endif

    return token;
}

// * - Errors -
static void
lexical_error(NovisLexer *self, char *format, ...)
{
    char buffer[1024];
    va_list arg;
    if (format) {
        va_start(arg, format);
        vsnprintf(buffer, sizeof(buffer), format, arg);
        va_end(arg);
    }

    novis_error(
        self->src->name,
        self->line,
        self->start,
        "Lexical Error",
        buffer
    );
}

// * - Lexing -

static NovisToken
identifier(NovisLexer *self)
{
    while (is_identifier(current(self)))
        get_raw(self);

    return create_token(self, TOKEN_IDENTIFIER);
}

static NovisToken
number(NovisLexer *self)
{
    while (is_digit(current(self)))
        get_raw(self);

    return create_token(self, TOKEN_NUMBER);
}

static NovisToken
string(NovisLexer *self)
{
    int c = get_raw(self);

    while (!is_at_end(self) && c != '"')
        c = get_raw(self);

    if (c == '\n') { // we hit newline but there wasn't terminator?
        lexical_error(self, "unterminated string");
        return create_token(self, TOKEN_ERROR);
    }

    return create_token(self, TOKEN_STRING);
}

NovisToken
novis_lex(NovisLexer *self)
{
    int c;

    self->start = self->end;
    c = get_raw(self);

    switch (c) {
        // End-of-file
        case EOF:
            return create_token(self, TOKEN_EOF);
        // ignore meaningless
        case ' ': case '\t': case '\n':
        case '\v': case '\f': case '\r':
            return novis_lex(self);
        // One character tokens
        case '(':
            return create_token(self, TOKEN_LPAREN);
        case ')':
            return create_token(self, TOKEN_RPAREN);
        case '+':
            return create_token(self, TOKEN_PLUS);
        case '*':
            return create_token(self, TOKEN_STAR);
        case '/':
            return create_token(self, TOKEN_SLASH);
        case '.':
            return create_token(self, TOKEN_DOT);
        case ',':
            return create_token(self, TOKEN_COMMA);
        case '"':
            return string(self);
        // One or two character long
        case '-': {
            if (match(self, '>'))
                return create_token(self, TOKEN_ARROW_RIGHT);

            return create_token(self, TOKEN_MINUS);
        }
        case '<': {
            if (match(self, '-'))
                return create_token(self, TOKEN_ARROW_LEFT);

            if (match(self, '='))
                return create_token(self, TOKEN_LESS_EQUAL);

            return create_token(self, TOKEN_GREATER_EQUAL);
        }
        case '>': {
            if (match(self, '='))
                return create_token(self, TOKEN_GREATER_EQUAL);

            return create_token(self, TOKEN_GREATER);
        }
        case '=': {
            if (match(self, '='))
                return create_token(self, TOKEN_EQUAL_EQUAL);

            return create_token(self, TOKEN_EQUAL);
        }
        // Keywords
        case 'f': {
            if (match_word(self, "alse")) // false
                return create_token(self, TOKEN_FALSE);

            if (match_word(self, "unc")) // func
                return create_token(self, TOKEN_FUNC);
        } break;
        case 't': {
            if (match_word(self, "rue")) // true
                return create_token(self, TOKEN_TRUE);
        } break;
        default: break;
    }

    if (is_identifier(c)) // is it identifier?
        return identifier(self);
    else if (is_digit(c)) // having a digit?
        return number(self);

    // Otherwise, we don't know what it is
    lexical_error(self, "illegal character %c", c);
    return create_token(self, TOKEN_ERROR);
}

void
novis_init_lexer(NovisLexer *lexer, NovisInput *src)
{
    lexer->current = 0;
    lexer->start = 0;
    lexer->end = 0;
    lexer->line = 0;
    lexer->src = src;
}
