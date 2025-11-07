/**
 * @file
 *
 * This file contains the code for the CountOperators traversal.
 * The traversal has the uid: CO
 *
 * Counts occurrences of the five arithmetic operators (add, sub, mul, div, mod)
 * and stores the results in the root node attributes.
 *
 */

#include "ccn/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/trav_data.h"
#include "ccngen/enum.h"

/**
 * Initialize traversal data - reset all counters to 0
 */
void COinit()
{
    struct data_co *data = DATA_CO_GET();
    data->add_count = 0;
    data->sub_count = 0;
    data->mul_count = 0;
    data->div_count = 0;
    data->mod_count = 0;
}

/**
 * Cleanup traversal data
 */
void COfini()
{
    // Nothing to clean up
}

/**
 * @fn CObinop
 * Count the operator and continue traversal
 */
node_st *CObinop(node_st *node)
{
    struct data_co *data = DATA_CO_GET();

    // Count the operator
    switch (BINOP_OP(node))
    {
    case BO_add:
        data->add_count++;
        break;
    case BO_sub:
        data->sub_count++;
        break;
    case BO_mul:
        data->mul_count++;
        break;
    case BO_div:
        data->div_count++;
        break;
    case BO_mod:
        data->mod_count++;
        break;
    default:
        // Not an arithmetic operator, don't count
        break;
    }

    // Continue traversing children
    TRAVchildren(node);

    return node;
}

/**
 * @fn COcompilationunit
 * Store the collected counts in the root node attributes
 */
node_st *COcompilationunit(node_st *node)
{
    struct data_co *data = DATA_CO_GET();

    // First traverse children to count operators
    TRAVchildren(node);

    // Then store the counts in the root node
    COMPILATIONUNIT_ADD_COUNT(node) = data->add_count;
    COMPILATIONUNIT_SUB_COUNT(node) = data->sub_count;
    COMPILATIONUNIT_MUL_COUNT(node) = data->mul_count;
    COMPILATIONUNIT_DIV_COUNT(node) = data->div_count;
    COMPILATIONUNIT_MOD_COUNT(node) = data->mod_count;

    return node;
}
