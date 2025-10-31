fibs:
    esr 1
    iloadc_0
    istore 1
14_while:
    iload_1
    iload_0
    ilt
    branch_f 15_end
    isrg
    isrg
    iload_1
    jsre 3
    jsre 0
    iload_1
    iloadc_1
    iadd
    iload_0
    ilt
    branch_f 16_end
    isrg
    iloadc_1
    jsre 1
16_end:
    iinc_1 1               ; add -> inc
    jump 14_while
15_end:
    isrg
    iloadc_1
    jsre 2
    return

primes:
    esr 2
    iloadc_0
    istore 1
    iloadc 0
    istore 2
10_while:
    iload_1
    iload_0
    ilt
    branch_f 11_end
    isrl
    iload_2
    jsr 1 __primes_isprime_local
    branch_f 12_end
    isrg
    iload_2
    jsre 0
    iinc_1 1               ; add -> inc
    iload_1
    iload_0
    ilt
    branch_f 13_end
    isrg
    iloadc_1
    jsre 1
13_end:
12_end:
    iinc_1 2               ; add -> inc
    jump 10_while
11_end:
    isrg
    iloadc_1
    jsre 2
    return

__primes_isprime_local:
    esr 2
    iload_0
    iloadc 0
    ilt
    branch_f 9_end
    bloadc_f
    breturn
9_end:
    iload_0
    iloadc 0
    ieq
    branch_f 8_end
    bloadc_t
    breturn
8_end:
    iload_0
    iloadc 0
    irem
    iloadc_0
    ieq
    branch_f 7_end
    bloadc_f
    breturn
7_end:
    iloadc 1
    istore 1
    bloadc_f
    bstore 2
1_while:
    iload_1
    iload_1
    imul
    iload_0
    ile
    branch_f 5_false_expr
    bload_2
    bnot
    jump 6_end
5_false_expr:
    bloadc_f
6_end:
    branch_f 2_end
    iload_0
    iload_1
    irem
    iloadc_0
    ieq
    branch_f 3_else
    bloadc_t
    bstore 2
    jump 4_end
3_else:
    iinc 1 0               ; add -> inc
4_end:
    jump 1_while
2_end:
    bload_2
    bnot
    breturn

.const int 2
.const int 3
.exportfun "fibs" void int fibs
.exportfun "primes" void int primes
.importfun "printInt" void int
.importfun "printSpaces" void int
.importfun "printNewlines" void int
.importfun "fib" int int
