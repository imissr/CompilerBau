/**
 * @file
 *
 * Traversal: StrengthReduction
 * UID      : SR
 *
 *
 */

#include "ccngen/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/enum.h"

/**
 * @fn SRbinop
 */
node_st *SRbinop(node_st *node)
{
    TRAVleft(node);
    TRAVright(node);

    if (BINOP_OP(node) != BO_mul) {
        return node;
    }

    node_st *left = BINOP_LEFT(node);
    node_st *right = BINOP_RIGHT(node);

    if (NODE_TYPE(left) == NT_NUM && NUM_VAL(left) == 2 &&
        NODE_TYPE(right) == NT_VAR) {
        /* 2 * k -> k + k: each child must own a separate variable node. */
        node_st *copy = CCNcopy(right);
        BINOP_LEFT(node) = CCNfree(left);
        BINOP_LEFT(node) = copy;
        BINOP_OP(node) = BO_add;
    } else if (NODE_TYPE(left) == NT_VAR &&
               NODE_TYPE(right) == NT_NUM && NUM_VAL(right) == 2) {
        /* k * 2 -> k + k. Keep the binop and its source location. */
        node_st *copy = CCNcopy(left);
        BINOP_RIGHT(node) = CCNfree(right);
        BINOP_RIGHT(node) = copy;
        BINOP_OP(node) = BO_add;
    }

    return node;
}

