CC = gcc
# Release build flags
RELFLAGS = -Wall -I./include/ -O3
# Debug flags
DBGFLAGS = -Wall -I./include/ -O0 -g
EXEC = novis

SRCMODULES = core/errors.c core/io.c core/memory.c \
			 compiler/lexer.c

OBJMODULES = $(SRCMODULES:.c=.o)

.DEFAULT_GOAL := all
.PHONY: all clean build install debug

DEBUG ?= 0
DBG_OPTIONS ?= -D NV_DBG;
ifeq ($(DEBUG), 1)
    CFLAGS := $(DBGFLAGS) $(DBG_OPTIONS)
else
    CFLAGS := $(RELFLAGS)
endif

%.o: %.c %.h
	$(CC) $(CFLAGS) -c $< -o ./bin/$@

build: novis.c $(OBJMODULES)
	$(CC) $(CFLAGS) $^ -o ./bin/$(EXEC)

clean:
	rm -f ./$(OBJMODULES)

install:
	install ./bin/$(EXEC) /usr/local/bin

all:
	mkdir -p bin
	make build
	make install
	make clean
