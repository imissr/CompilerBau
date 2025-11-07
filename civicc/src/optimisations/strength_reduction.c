/**
 * @file
 *
 * This file contains the code for the StrengthReduction traversal.
 * The traversal has the uid: SR
 *
 * Implements strength reduction optimization:
 * - 2 * k -> k + k
 * - k * 2 -> k + k
 *
 */

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
            // Create a copy of the right operand
            node_st *right_copy = CCNcopy(right);

            // Change the node from multiplication to addition
            BINOP_OP(node) = BO_add;

            // Replace left child (the constant 2) with the copy
            CCNfree(left); // Free the '2' num node
            BINOP_LEFT(node) = right_copy;

            return node;
        }

        // Case 2: k * 2 -> k + k
        if (NODE_TYPE(right) == NT_NUM && NUM_VAL(right) == 2)
        {
            // Create a copy of the left operand
            node_st *left_copy = CCNcopy(left);

            // Change the node from multiplication to addition
            BINOP_OP(node) = BO_add;

            // Replace right child (the constant 2) with the copy
            CCNfree(right); // Free the '2' num node
            BINOP_RIGHT(node) = left_copy;

            return node;
        }
    }

    return node;
}
