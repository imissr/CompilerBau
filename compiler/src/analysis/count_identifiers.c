#include "ccngen/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "ccngen/trav_data.h"
#include "palm/memory.h"
#include "stdio.h"
#include "palm/str.h" // for STRcpy

void CIinit(void) {
    // creat a hashtable where we can store names and their counters
    // Data_ci_get data structure of the running traversal data -> id 
    // HTnew_String creates a new hash table with the specified size (64 in this case) 
    // short wrapper provided by coconut to create a new hash table for storing identifiers and their counters.
    DATA_CI_GET()->ids = HTnew_String(64);

}

// CountName is a function that takes a variable name as input and updates its count in the hash table.
static void CountName( char *name){
    htable_stptr ids = DATA_CI_GET()->ids;
    int *count = HTlookup(ids, name);
    if(count == NULL){
        // MEMmalloc wrapper provided by coconut to allocate memory.
        count = MEMmalloc(sizeof(int));
        *count = 0;
        HTinsert(ids, STRcpy(name), count);
    } else {
        (*count)++;
    }
}

// free the memory! 
void CIfini(void)
{
    htable_st *ids = DATA_CI_GET()->ids;

    for (htable_iter_st *it = HTiterate(ids); it != NULL; it = HTiterateNext(it)) {
        MEMfree(HTiterKey(it));
        MEMfree(HTiterValue(it));
    }
    HTdelete(ids);
}



node_st *CIprogram(node_st *node) {
    // traverse the childern (which run functions below for every identifier)
    TRAVchildren(node);

    printf("// Identifiers and their counts:");
    htable_stptr ids = DATA_CI_GET()->ids;



    for(htable_iter_st *iterator = HTiterate(ids); iterator != NULL; iterator = HTiterateNext(iterator)) {
      
        printf(" %s: %d ", (char *)HTiterKey(iterator), *(int *)HTiterValue(iterator));

        
    }
    printf("\n");
    return node;

}

node_st *CIvar(node_st *node) {
    // VAR_NAME(node) is a macro that retrieves the name of the variable from the node structure.
    CountName(VAR_NAME(node));
    return node;
}

node_st *CIvarlet(node_st *node) {
    // VARLET_NAME(node) is a macro that retrieves the name of the variable from the node structure.
    CountName(VARLET_NAME(node));
    return node;
}
