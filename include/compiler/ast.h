#ifndef NV_AST_H
#define NV_AST_H

#include "lexer.h"

typedef struct NovisAstUnary NovisAstUnary;
typedef struct NovisAstBinary NovisAstBinary;
typedef struct NovisAstPrimary NovisAstPrimary;

struct NovisAstNode {
    enum {
        NODE_PRIMARY = 0,
        NODE_UNARY,
        NODE_BINARY,
    } type;

    union {
        NovisAstPrimary *primary;
        NovisAstUnary   *unary;
        NovisAstBinary  *binary;
    };
};

struct NovisAstPrimary {
    NovisToken *tok;
};

struct NovisAstUnary {
    NovisToken *op;
    struct novis_ast_node_t *right;
};

struct NovisAstBinary {
    NovisToken *op;
    struct novis_ast_node_t *left;
    struct novis_ast_node_t *right;
};

typedef struct NovisAstNode NovisAstNode;

#endif
