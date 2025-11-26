#include "ccn/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/trav_data.h"
#include "ccngen/enum.h"

#include "palm/hash_table.h"
#include "palm/memory.h"
#include "palm/str.h"
#include <stdio.h>

/**
 * Initialize traversal data - create the hash table
 */
void CIinit()
{
    struct data_ci *data = DATA_CI_GET();
    data->table = HTnew_String(256);
}

/**
 * Cleanup traversal data - free counters and keys and delete table
 */
void CIfini()
{
    struct data_ci *data = DATA_CI_GET();
    if (!data->table)
        return;

    for (htable_iter_st *iter = HTiterate(data->table); iter; iter = HTiterateNext(iter))
    {
        char *key = (char *)HTiterKey(iter);
        int *val = (int *)HTiterValue(iter);
        if (val)
            MEMfree(val);
        if (key)
            MEMfree(key);
    }

    HTdelete(data->table);
    data->table = NULL;
}

/**
 * @fn CIvar
 * Count a Var occurrence
 */
node_st *CIvar(node_st *node)
{
    struct data_ci *data = DATA_CI_GET();
    const char *name = VAR_NAME(node);
    if (!name)
        return node;

    int *cnt = (int *)HTlookup(data->table, (void *)name);
    if (!cnt)
    {
        int *nptr = MEMmalloc(sizeof(int));
        *nptr = 1;
        char *keycopy = STRcpy(name);
        HTinsert(data->table, (void *)keycopy, (void *)nptr);
    }
    else
    {
        (*cnt)++;
    }

    return node;
}

/**
 * @fn CIvarlet
 * Count a VarLet occurrence (left-hand side variable)
 */
node_st *CIvarlet(node_st *node)
{
    struct data_ci *data = DATA_CI_GET();
    const char *name = VARLET_NAME(node);
    if (!name)
        return node;

    int *cnt = (int *)HTlookup(data->table, (void *)name);
    if (!cnt)
    {
        int *nptr = MEMmalloc(sizeof(int));
        *nptr = 1;
        char *keycopy = STRcpy(name);
        HTinsert(data->table, (void *)keycopy, (void *)nptr);
    }
    else
    {
        (*cnt)++;
    }

    return node;
}

/**
 * @fn CIcompilationunit
 * At root: print identifier counts collected in travdata
 */
node_st *CIcompilationunit(node_st *node)
{
    struct data_ci *data = DATA_CI_GET();

    // Traverse children first
    TRAVchildren(node);

    if (!data->table)
        return node;

    printf("Identifier counts:\n");
    for (htable_iter_st *iter = HTiterate(data->table); iter; iter = HTiterateNext(iter))
    {
        char *key = (char *)HTiterKey(iter);
        int *val = (int *)HTiterValue(iter);
        if (key && val)
        {
            printf("  %s: %d\n", key, *val);
        }
    }

    return node;
}
