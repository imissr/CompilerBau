/**
 * @file
 *
 * This file contains the code for the Print traversal.
 * The traversal has the uid: PRT
 *
 *
 */

#include "ccn/ccn.h"
#include "ccngen/ast.h"
#include "ccngen/trav.h"
#include "palm/dbug.h"

/**
 * @fn PRTcompilationunit
 */
node_st *PRTcompilationunit(node_st *node)
{
  if (COMPILATIONUNIT_ADD_COUNT(node) != 0 ||
      COMPILATIONUNIT_SUB_COUNT(node) != 0 ||
      COMPILATIONUNIT_MUL_COUNT(node) != 0 ||
      COMPILATIONUNIT_DIV_COUNT(node) != 0 ||
      COMPILATIONUNIT_MOD_COUNT(node) != 0)
  {

    printf("// Operator Statistics:\n");
    printf("//   Addition (+):       %d\n", COMPILATIONUNIT_ADD_COUNT(node));
    printf("//   Subtraction (-):    %d\n", COMPILATIONUNIT_SUB_COUNT(node));
    printf("//   Multiplication (*): %d\n", COMPILATIONUNIT_MUL_COUNT(node));
    printf("//   Division (/):       %d\n", COMPILATIONUNIT_DIV_COUNT(node));
    printf("//   Modulo (%%):         %d\n", COMPILATIONUNIT_MOD_COUNT(node));
    printf("//\n");
  }

  TRAVstmts(node);
  return node;
}

/**
 * @fn PRTprogram
 */
node_st *PRTprogram(node_st *node)
{
  TRAVstmts(node);
  return node;
}

/**
 * @fn PRTstmts
 */
node_st *PRTstmts(node_st *node)
{
  TRAVstmt(node);
  TRAVnext(node);
  return node;
}

/**
 * @fn PRTassign
 */
node_st *PRTassign(node_st *node)
{

  if (ASSIGN_LET(node) != NULL)
  {
    TRAVlet(node);
    printf(" = ");
  }

  TRAVexpr(node);
  printf(";\n");

  return node;
}

/**
 * @fn PRTbinop
 */
node_st *PRTbinop(node_st *node)
{
  char *tmp = NULL;
  printf("( ");

  TRAVleft(node);

  switch (BINOP_OP(node))
  {
  case BO_add:
    tmp = "+";
    break;
  case BO_sub:
    tmp = "-";
    break;
  case BO_mul:
    tmp = "*";
    break;
  case BO_div:
    tmp = "/";
    break;
  case BO_mod:
    tmp = "%";
    break;
  case BO_lt:
    tmp = "<";
    break;
  case BO_le:
    tmp = "<=";
    break;
  case BO_gt:
    tmp = ">";
    break;
  case BO_ge:
    tmp = ">=";
    break;
  case BO_eq:
    tmp = "==";
    break;
  case BO_ne:
    tmp = "!=";
    break;
  case BO_or:
    tmp = "||";
    break;
  case BO_and:
    tmp = "&&";
    break;
  case BO_NULL:
    DBUG_ASSERT(false, "unknown binop detected!");
  }

  printf(" %s ", tmp);

  TRAVright(node);

  printf(")(%d:%d-%d)", NODE_BLINE(node), NODE_BCOL(node), NODE_ECOL(node));

  return node;
}

/**
 * @fn PRTvarlet
 */
node_st *PRTvarlet(node_st *node)
{
  printf("%s(%d:%d)", VARLET_NAME(node), NODE_BLINE(node), NODE_BCOL(node));
  return node;
}

/**
 * @fn PRTvar
 */
node_st *PRTvar(node_st *node)
{
  printf("%s", VAR_NAME(node));
  return node;
}

/**
 * @fn PRTnum
 */
node_st *PRTnum(node_st *node)
{
  printf("%d", NUM_VAL(node));
  return node;
}

/**
 * @fn PRTfloat
 */
node_st *PRTfloat(node_st *node)
{
  printf("%f", FLOAT_VAL(node));
  return node;
}

/**
 * @fn PRTbool
 */
node_st *PRTbool(node_st *node)
{
  char *bool_str = BOOL_VAL(node) ? "true" : "false";
  printf("%s", bool_str);
  return node;
}
