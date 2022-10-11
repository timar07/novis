#ifndef NV_VM_H
#define NV_VM_H

#include <stdint.h>

typedef enum {
    VM_CONSTANT = 0,
    VM_NAME,
    VM_ADD,
    VM_SUB,
    VM_MUL,
    VM_DIV,
    VM_HALT,
} NovisVmOpcode;

const char *
novis_disassembly(NovisVmOpcode opcode);

void
vm_run(uint8_t *chunk);

#endif
