main:
    isrg
    isrg
    iloadc 0
    iloadc 1
    jsre 6
    jsre 0
    isrg
    iloadc_1
    jsre 5
    isrg
    isrg
    iloadc 2
    jsre 7
    jsre 0
    isrg
    iloadc_1
    jsre 5
    isrg
    isrg
    iloadc 3
    jsre 8
    jsre 0
    isrg
    iloadc_1
    jsre 5
    isrg
    isrg
    iloadc 4
    jsre 9
    branch_f 3_false_expr
    iloadc_1
    jump 4_end
3_false_expr:
    iloadc_0
4_end:
    jsre 0
    isrg
    iloadc_1
    jsre 5
    isrg
    isrg
    iloadc 5
    jsre 9
    branch_f 1_false_expr
    iloadc_1
    jump 2_end
1_false_expr:
    iloadc_0
2_end:
    jsre 0
    isrg
    iloadc_1
    jsre 5
    iloadc_0
    ireturn

.const int 48
.const int 18
.const int 5
.const int 10
.const int 29
.const int 42
.exportfun "main" int main
.importfun "printInt" void int
.importfun "printFloat" void float
.importfun "scanInt" int
.importfun "scanFloat" float
.importfun "printSpaces" void int
.importfun "printNewlines" void int
.importfun "gcd" int int int
.importfun "fac" int int
.importfun "fib" int int
.importfun "isprime" bool int
