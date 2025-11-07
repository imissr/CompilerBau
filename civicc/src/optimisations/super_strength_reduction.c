

#include "ccn/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/trav_data.h"
#include "ccngen/enum.h"
#include "global/globals.h"

/**
 * Initialize traversal data with max_factor from global settings
 */
void SSRinit()
{
    struct data_ssr *data = DATA_SSR_GET();
    data->max_factor = global.max_strength_reduction_factor;
}

/**
 * Cleanup traversal data
 */
void SSRfini()
{
   
}

/**
 * @fn SSRbinop
 */
node_st *SSRbinop(node_st *node)
{
    TRAVleft(node);
    TRAVright(node);

    // Only apply if max_factor is set (> 0)
    struct data_ssr *data = DATA_SSR_GET();
    if (data->max_factor <= 0)
    {
        return node; 
    }

    // Check if this is a multiplication operation
    if (BINOP_OP(node) == BO_mul)
    {
        node_st *left = BINOP_LEFT(node);
        node_st *right = BINOP_RIGHT(node);
        int factor = -1;
        node_st *expr = NULL;

        // Case 1: n * k where n is a constant
        if (NODE_TYPE(left) == NT_NUM)
        {
            factor = NUM_VAL(left);
            expr = right;
        }
        // Case 2: k * n where n is a constant
        else if (NODE_TYPE(right) == NT_NUM)
        {
            factor = NUM_VAL(right);
            expr = left;
        }

        // Apply transformation if factor is in range [2, max_factor]
        if (factor >= 2 && factor <= data->max_factor && expr != NULL)
        {
            // Create: expr + expr + ... + expr (factor times)
            node_st *result = CCNcopy(expr);

            for (int i = 1; i < factor; i++)
            {
                node_st *expr_copy = CCNcopy(expr);
                result = ASTbinop(result, expr_copy, BO_add);
            }

            // Free the old multiplication node
            CCNfree(node);
            
            node = result;
        }
    }

    return node;
}
