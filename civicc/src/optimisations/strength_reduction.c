
#include "ccn/ccn.h"
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

    // Check if this is a multiplication operation
    if (BINOP_OP(node) == BO_mul)
    {
        node_st *left = BINOP_LEFT(node);
        node_st *right = BINOP_RIGHT(node);

        // Case 1: 2 * k -> k + k
        if (NODE_TYPE(left) == NT_NUM && NUM_VAL(left) == 2)
        {

            node_st *right_copy = CCNcopy(right);

            BINOP_OP(node) = BO_add;

            CCNfree(left); 
            BINOP_LEFT(node) = right_copy;

            return node;
        }

        // Case 2: k * 2 -> k + k
        if (NODE_TYPE(right) == NT_NUM && NUM_VAL(right) == 2)
        {
            node_st *left_copy = CCNcopy(left);

            BINOP_OP(node) = BO_add;

            CCNfree(right); 
            BINOP_RIGHT(node) = left_copy;

            return node;
        }
    }

    return node;
}
