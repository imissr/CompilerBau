# Assignment 2 Implementation Plan

**Goal:** Complete Assignments 2.1–2.4 from `course/assignments/assign_2.pdf`.

**Architecture:** Keep the existing subtraction and factor-2 traversals in
`Optimisations`. Add a separate parameterised super-strength-reduction traversal.
Introduce a `Program` root; collect source operator counts with traversal data
immediately after parsing, store them on that root, and print them when valid.

**Tech stack:** C, CoCoNut DSL, Flex/Bison, CMake/CTest.

## Decisions and constraints

- Work in the user's current checkout; preserve unrelated deleted course files.
- Use `--strength-reduction-max N` / `-r N`, default 2, with an integer minimum
  of 2. Expand integer literal factors from 3 through N on either side of a
  variable. Existing factor-2 reduction always runs first.
- Do not duplicate expressions, booleans, or numeric literals as operands.
  The starter AST has no variable type information; variable type checks belong
  to a later assignment.
- Keep statistics for the source, before demo and optimisation traversals.
- Keep counts in traversal data and AST attributes, never global counters.
- A validity attribute distinguishes unavailable counts from five zero counts.
- The full public CiviC suites (`basic`, `nested_funs`, `arrays`) already fail
  because the starter parser does not implement the full language. Record those
  separately from the assignment regressions.
- No packaging, commit, or submission is requested.

## Task 1: Source statistics and dedicated root (2.3, 2.4)

- [x] Add CMake regressions for all five operators, nesting, multiple statements,
  zero counts, source counts surviving optimisation, and printing before counting.
- [x] Run the new tests against the existing compiler and confirm missing counts.
- [x] Define `Program(stmts)` with five integer counters and a validity flag in
  `compiler/src/main.ccn`; return it from `parser.y`.
- [x] Implement `OCprogram` / `OCbinop` with CoCoNut traversal data in
  `compiler/src/analysis/count_operators.c`, before Demo/Optimisations.
- [x] Add `PRTprogram`, move the integer-sum summary to `SIprogram`, and update
  the existing strength-reduction comparison to ignore the statistics header.
- [x] Build and run the statistics and existing reduction tests.

## Task 2: Configurable super strength reduction (2.2)

- [x] Add regressions for factors on either side, the inclusive limit, default
  behaviour, nested matches, untouched non-variable operands and factors 0/1,
  and malformed/missing/overflowing command-line limits.
- [x] Run the new tests and confirm the option is not yet supported.
- [x] Add the validated CLI option and default to `main.c` and `global/globals.*`.
- [x] Add `SSRbinop` to `compiler/src/optimisations/super_strength_reduction.c`
  and register it after `StrengthReduction`. Retain the original binop/source
  location and use independent variable copies for the addition chain.
- [x] Build and run all Assignment 2 tests with AddressSanitizer enabled.

## Task 3: Documentation and final verification

- [x] Document each pass, the option, examples, statistics timing and test commands.
- [x] Check a fresh build, demo inputs, help/phase structure, and the full suite.
- [x] Review the diff for ownership errors, stale root assumptions, and changes
  outside the assignment; report remaining public-suite failures explicitly.

## Verification result

- Existing and fresh Debug builds succeeded with AddressSanitizer enabled.
- All five Assignment 2 CTest groups passed in both builds.
- Both demo inputs ran successfully; phase structure includes counting and both
  reduction traversals.
- Full suite: five groups passed; the existing `basic`, `nested_funs`, and
  `arrays` failures remain.
- Independent review found no functional issues; its documentation follow-up
  was completed in the READMEs.
- `git diff --check` passed. No unrelated user deletions were changed.
