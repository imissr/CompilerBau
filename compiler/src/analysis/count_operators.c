/** Count source arithmetic operators using traversal-local CoCoNut data. */
#include "ccngen/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/trav_data.h"

/* CoCoNut initialises the integer traversal data to zero on every traversal. */
void OCinit(void) {}
void OCfini(void) {}

node_st *OCprogram(node_st *node)
{
    TRAVstmts(node);

    struct data_oc *data = DATA_OC_GET();
    PROGRAM_ADD_COUNT(node) = data->add_count;
    PROGRAM_SUB_COUNT(node) = data->sub_count;
    PROGRAM_MUL_COUNT(node) = data->mul_count;
    PROGRAM_DIV_COUNT(node) = data->div_count;
    PROGRAM_MOD_COUNT(node) = data->mod_count;
    PROGRAM_COUNTS_VALID(node) = true;
    return node;
}

node_st *OCbinop(node_st *node)
{
    struct data_oc *data = DATA_OC_GET();
    switch (BINOP_OP(node)) {
    case BO_add: data->add_count++; break;
    case BO_sub: data->sub_count++; break;
    case BO_mul: data->mul_count++; break;
    case BO_div: data->div_count++; break;
    case BO_mod: data->mod_count++; break;
    default: break;
    }

    TRAVchildren(node);
    return node;
}
