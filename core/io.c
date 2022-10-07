#include "core/io.h"
#include "core/memory.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdio.h>

static novis_input_t *src; /* source file */

static size_t get_file_len(FILE *f)
{
	fseek(f, 0, SEEK_END);
	return ftell(f);
}

static void
set_input_name(const char *name)
{
	strcpy(src->name, name);
}

novis_input_t *
novis_read(const char *fname)
{
	FILE *f;
	size_t flen;
	src = novis_alloc(sizeof(novis_input_t));

	if ((f = fopen(fname, "r")) == NULL) {
		fprintf(stderr, "cannot open file: %s\n", fname);
		exit(1);
	}

	flen = get_file_len(f);
	fseek(f, 0, SEEK_SET);
	src->raw = novis_alloc(flen);
	src->len = flen/sizeof(char);
	set_input_name(fname);

	if (src->raw != NULL) {
		fread(src->raw, 1, flen, f);
	} else {
		fprintf(stderr, "file source malloc failed\n");
		exit(1);
	}

	fclose(f);
	return src;
}
