# Assignment 4.2 – AST Definition TODO

Extend `compiler/src/main.ccn` to cover the full CiviC language.
Submit: `main.ccn` + `ast.png` — **due Friday, November 27, 2026**.

Reference: `docs/reference/civic-language.pdf` (Fig. 1–7).

## Required – core language

### Enum
- [ ] Type enum: `void`, `bool`, `int`, `float`

### Top level (Fig. 1)
- [ ] `Program` holds a list of **declarations** (currently holds `Stmts`)
- [ ] Declaration list node
- [ ] Function definition: return type, name, params, body, `export` flag
- [ ] Function declaration (`extern`): return type, name, params, no body
- [ ] Global definition: type, name, optional init expr, `export` flag
- [ ] Global declaration (`extern`): type, name
- [ ] Parameter list
- [ ] Parameter: type, name

### Function body (Fig. 2)
- [ ] Function body: local var-decls, then statements (both may be empty)
- [ ] Local var-decl list
- [ ] Local var-decl: type, name, optional init expr

### Statements (Fig. 2)
- [x] Assignment (`Assign`)
- [ ] Procedure call `foo(a, b);`
- [ ] `if`: condition, then-block, optional else-block
- [ ] `while`: condition, block
- [ ] `do … while`: block, condition
- [ ] `for`: induction var name, start, stop, optional step, block
- [ ] `return`: optional expression

### Expressions (Fig. 3)
- [x] `Binop`, `Var`, `Num`, `Float`, `Bool`
- [ ] Unary op node (`MonOpType` enum exists, no node uses it)
- [ ] Cast `(int) e`: target type + expression
- [ ] Function call `foo(a, b)`: name + argument list
- [ ] Argument (expression) list

### Nodesets
- [ ] Add all new statement nodes to `Stmt`, all new expression nodes to `Expr`
- [ ] Rework `Link`: should cover what a variable can refer to
      (param, local var-decl, global def/dec, for-loop variable)

## Extensions (after the core)

### Ext. 1 – Nested functions (Fig. 4)
- [ ] Function body gets a list of local function definitions

### Ext. 2 – Arrays (Fig. 6)
- [ ] Array dimensions (identifiers) on global declarations and parameters
- [ ] Array dimensions (expressions) on global definitions and local var-decls
- [ ] Array literal initialiser `[e1, e2, …]`
- [ ] Index expression on `Var` (read) and on `VarLet` (assignment target)

### Ext. 3 – Multi-dimensional arrays (Fig. 7)
- [ ] Every dimension/index from Ext. 2 becomes a list; array literals can nest

## Cleanup
- [ ] Decide on `Program` attributes `add_count … counts_valid`
      (assignment-2 leftovers; still used by `CountOperators`)
- [ ] Update/stub `scanparse/parser.y` and traversals that break when `Program` changes

## Generate ast.png
- [ ] `cmake --build build/compiler` (regenerates `ccngen/ast.dot`)
- [ ] `cmake --build build/compiler --target dot` → `build/compiler/ast.png`
