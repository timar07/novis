#include "vm/vm_api.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

uint8_t stack[1024];

void
vm_run(uint8_t *chunk)
{
    size_t ip = 0;
    size_t sp = 0;

    while (chunk[ip] != VM_HALT) {
        printf("%s\n", novis_disassembly(chunk[ip]));
        switch (chunk[ip]) {
            case VM_CONSTANT: {
                stack[sp++] = chunk[ip+1];
                ip += 2;
            } break;
            case VM_ADD: {
                uint8_t val;
                val = stack[sp];
                val += stack[--sp];
                stack[++sp] = val;
            } break;
            case VM_MUL: {
                uint8_t val;
                val = stack[sp--];
                val *= + stack[sp--];
                stack[sp] = val;
            } break;
            case VM_HALT: {
                goto end;
            }
        }

        ip++;
    }

    end:
        printf("RESULT: %d\n", stack[sp]);
}
