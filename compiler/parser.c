#include "compiler/parser.h"
#include "compiler/lexer.h"
#include "compiler/debug_macros.h"
#include "vm/vm_api.h"
#include "core/memory.h"
#include "core/errors.h"
#include <stdbool.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct {
    NovisLexer *lexer;
    NovisToken  current;
    NovisToken  prev;
    size_t      ip;
} NovisParser;

uint8_t chunk[1024];

// * - Errors -
static void
syntax_error(NovisParser *self, const char *format, ...)
{
    char buffer[1024];
    va_list arg;
    if (format) {
        va_start(arg, format);
        vsnprintf(buffer, sizeof(buffer), format, arg);
        va_end(arg);
    }

    novis_error(
        self->lexer->src->name,
        self->lexer->line,
        self->lexer->end,
        "Syntax Error",
        buffer
    );
}

// * - Utils -

static NovisToken
get_token(NovisParser *self)
{
    self->prev = self->current;
    self->current = novis_lex(self->lexer);
    return self->current;
}

static bool
require(NovisParser *self, NovisTokenTag expected)
{
    return get_token(self).tag == expected;
}

static bool
match_token(NovisParser *self, NovisTokenTag expected)
{
    return get_token(self).tag == expected;
}

static void emit_byte(NovisParser *self, NovisVmOpcode opcode)
{
    chunk[self->ip++] = opcode;
}

static void emit_constant(NovisParser *self, uint8_t val)
{
    chunk[self->ip++] = VM_CONSTANT;
    chunk[self->ip++] = val;
}

// * - Grammar -

// Forwards
static void primary(NovisParser *self);
static void unary(NovisParser *self);
static void binary(NovisParser *self);

typedef void (*parse_func)(NovisParser *self);

typedef struct parse_rule {
    parse_func prefix;
    parse_func infix;
    int prec;
} ParseRule;

ParseRule rules[] = {
    [TOKEN_NUMBER] = {primary, NULL,    100},
    [TOKEN_STAR]   = {NULL,    binary,   20},
    [TOKEN_SLASH]  = {NULL,    binary,   20},
    [TOKEN_PLUS]   = {NULL,    binary,   10},
    [TOKEN_MINUS]  = {unary,   binary,   10},
    [TOKEN_EOF]    = {NULL,    NULL,      0}
};

#define get_rule(token_tag) (&rules[token_tag])

static void parse_with_prec(NovisParser *self, int prec)
{
    get_token(self);
    parse_func prefix = get_rule(self->current.tag)->prefix;

    if (prefix == NULL) {
        return syntax_error(
            self,
            "expected prefix operator, got '%s'",
            self->current.lexeme
        );
    }

    prefix(self);

    while (prec <= get_rule(self->current.tag)->prec) {
        get_token(self);
        parse_func infix = get_rule(self->current.tag)->infix;

        if (infix == NULL) break;
        infix(self);
    }
}

static void unary(NovisParser *self)
{
    NovisTokenTag op = self->current.tag;
    parse_with_prec(self, 30);

    switch (op) {
        case TOKEN_MINUS: emit_byte(self, VM_SUB); break; // TODO: negate
        default: printf("DEBUG\n");
    }
}

static void primary(NovisParser *self)
{
    // primary = LITERAL | "(" expr ")";
    switch (self->current.tag) {
        case TOKEN_LPAREN: {
            parse_with_prec(self, get_rule(self->current.tag)->prec);
            require(self, TOKEN_RPAREN);
        } break;
        case TOKEN_NUMBER: {
            // TODO: Emit value
            double num_val = strtod(self->current.lexeme, NULL);
            emit_constant(self, (uint8_t)num_val);
        } break;
        default:
            return syntax_error(
                self,
                "expected expression, got '%s'",
                self->current.lexeme
            );
    }
}

static void binary(NovisParser *self)
{
    NovisTokenTag op = self->current.tag;
    printf("BINARY: %u\n", op);
    ParseRule *rule = get_rule(op);
    parse_with_prec(self, rule->prec + 10);

    switch (op) {
        case TOKEN_PLUS:
            emit_byte(self, VM_ADD);
        break;
        case TOKEN_MINUS:
            emit_byte(self, VM_SUB);
        break;
        case TOKEN_STAR:
            emit_byte(self, VM_MUL);
        break;
        case TOKEN_SLASH:
            emit_byte(self, VM_DIV);
        break;
        default: break;
    }
}

void novis_parse(NovisLexer *lexer)
{
    NovisParser parser = {
        .lexer = lexer,
    };

    parse_with_prec(&parser, 10);
    emit_byte(&parser, VM_HALT);
    vm_run(chunk);
}
