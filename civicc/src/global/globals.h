#pragma once

#include <stdbool.h>
#include <stddef.h>

struct globals
{
    int line;
    int col;
    int verbose;
    char *input_file;
    char *output_file;
    int max_strength_reduction_factor; // Maximum factor for super strength reduction
};

extern struct globals global;
extern void GLBinitializeGlobals(void);
extern bool GLBstrengthReductionDisabled(void);
