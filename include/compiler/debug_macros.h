#ifndef NV_DEBUG_MACROS
#define NV_DEBUG_MACROS

#ifdef NV_DEBUG_LEXER
#   define LEXER_DEBUG(format, ...) printf(format, ##__VA_ARGS__);
#else
#   define LEXER_DEBUG(format, ...)
#endif

#endif
