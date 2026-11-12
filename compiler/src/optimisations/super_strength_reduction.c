/** Expand n * k and k * n into n additions of the variable k. */
#include "ccngen/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "global/globals.h"

node_st *SSRbinop(node_st *node)
{
    TRAVleft(node);
    TRAVright(node);

    if (BINOP_OP(node) != BO_mul) {
        return node;
    }

    node_st *constant = BINOP_LEFT(node);
    node_st *variable = BINOP_RIGHT(node);
    if (NODE_TYPE(constant) == NT_VAR && NODE_TYPE(variable) == NT_NUM) {
        constant = BINOP_RIGHT(node);
        variable = BINOP_LEFT(node);
    }
    if (NODE_TYPE(constant) != NT_NUM || NODE_TYPE(variable) != NT_VAR) {
        return node;
    }

    int factor = NUM_VAL(constant);
    /* The preceding StrengthReduction traversal already handles factor 2. */
    if (factor < 3 || factor > global.strength_reduction_max) {
        return node;
    }

    /* Keep the outer binop (including its source location). Every occurrence
     * owns a distinct variable node; sharing children would cause double frees. */
    node_st *sum = variable;
    for (int term = 1; term < factor - 1; ++term) {
        sum = ASTbinop(sum, CCNcopy(variable), BO_add);
    }

    CCNfree(constant);
    BINOP_LEFT(node) = sum;
    BINOP_RIGHT(node) = CCNcopy(variable);
    BINOP_OP(node) = BO_add;
    return node;
}
