gcd:
    esr 3
    iload_0
    istore 2
    iload_1
    istore 3
    iload_2
    iloadc_0
    ilt
    branch_f 16_end
    iload_2
    ineg
    istore 2
16_end:
    iload_3
    iloadc_0
    ilt
    branch_f 15_end
    iload_3
    ineg
    istore 3
15_end:
13_while:
    iload_3
    iloadc_0
    ine
    branch_f 14_end
    iload_2
    iload_3
    irem
    istore 4
    iload_3
    istore 2
    iload 4
    istore 3
    jump 13_while
14_end:
    iload_2
    ireturn

fac:
    esr 2
    iloadc_1
    istore 1
    iloadc_1
    istore 2
11_while:
    iload_1
    iload_0
    ile
    branch_f 12_end
    iload_2
    iload_1
    imul
    istore 2
    iinc_1 1               ; add -> inc
    jump 11_while
12_end:
    iload_2
    ireturn

fib:
    esr 4
    iload_0
    iloadc_0
    ile
    branch_f 10_end
    iloadc_0
    ireturn
10_end:
    iload_0
    iloadc_1
    ieq
    branch_f 9_end
    iloadc_1
    ireturn
9_end:
    iloadc 0
    istore 1
    iloadc_0
    istore 2
    iloadc_1
    istore 3
7_while:
    iload_1
    iload_0
    ile
    branch_f 8_end
    iload_2
    iload_3
    iadd
    istore 4
    iload_3
    istore 2
    iload 4
    istore 3
    iinc_1 1               ; add -> inc
    jump 7_while
8_end:
    iload_3
    ireturn

isprime:
    esr 1
    iload_0
    iloadc 0
    ilt
    branch_f 6_end
    bloadc_f
    breturn
6_end:
    iload_0
    iloadc 0
    ieq
    branch_f 5_end
    bloadc_t
    breturn
5_end:
    iload_0
    iloadc 0
    irem
    iloadc_0
    ieq
    branch_f 4_end
    bloadc_f
    breturn
4_end:
    iloadc 1
    istore 1
1_while:
    iload_1
    iload_1
    imul
    iload_0
    ile
    branch_f 2_end
    iload_0
    iload_1
    irem
    iloadc_0
    ieq
    branch_f 3_end
    bloadc_f
    breturn
3_end:
    iinc 1 0               ; add -> inc
    jump 1_while
2_end:
    bloadc_t
    breturn

.const int 2
.const int 3
.exportfun "gcd" int int int gcd
.exportfun "fac" int int fac
.exportfun "fib" int int fib
.exportfun "isprime" bool int isprime
