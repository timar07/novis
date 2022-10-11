#include "vm/vm_api.h"

const char * opcodes_strings[] = {
    "CONSTANT",
    "NAME",
    "ADD",
    "SUB",
    "MUL",
    "DIV",
};

const char *
novis_disassembly(NovisVmOpcode opcode)
{
    return opcodes_strings[opcode];
}
