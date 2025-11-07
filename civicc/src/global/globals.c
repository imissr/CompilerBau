#include "globals.h"
#include <stdbool.h>

struct globals global;

/*
 * Initialize global variables from globals.mac
 */

void GLBinitializeGlobals()
{
    global.col = 0;
    global.line = 0;
    global.input_file = NULL;
    global.output_file = NULL;
    global.max_strength_reduction_factor = 0;
}

/**
 * Gate function: returns true if basic strength reduction should run
 */
bool GLBstrengthReductionDisabled()
{
    return global.max_strength_reduction_factor == 0;
}
