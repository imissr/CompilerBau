printIntVec:
    esr 1
    iloadc_0
    istore 2
57_while:
    iload_2
    iload_0
    ilt
    branch_f 58_end
    isrg
    iload_2
    aload_1
    iloada
    jsre 0
    iload_2
    iloadc_1
    iadd
    iload_0
    ilt
    branch_f 59_end
    isrg
    iloadc_1
    jsre 2
59_end:
    iinc_1 2               ; add -> inc
    jump 57_while
58_end:
    isrg
    iloadc_1
    jsre 3
    return

printFloatVec:
    esr 1
    iloadc_0
    istore 2
54_while:
    iload_2
    iload_0
    ilt
    branch_f 55_end
    isrg
    iload_2
    aload_1
    floada
    jsre 1
    iload_2
    iloadc_1
    iadd
    iload_0
    ilt
    branch_f 56_end
    isrg
    iloadc_1
    jsre 2
56_end:
    iinc_1 2               ; add -> inc
    jump 54_while
55_end:
    isrg
    iloadc_1
    jsre 3
    return

printIntMat:
    esr 2
    iloadc_0
    istore 3
49_while:
    iload_3
    iload_0
    ilt
    branch_f 50_end
    iloadc_0
    istore 4
51_while:
    iload 4
    iload_1
    ilt
    branch_f 52_end
    isrg
    iload_3
    iload_1
    imul
    iload 4
    iadd
    aload_2
    iloada
    jsre 0
    iload 4
    iloadc_1
    iadd
    iload_1
    ilt
    branch_f 53_end
    isrg
    iloadc_1
    jsre 2
53_end:
    iinc_1 4               ; add -> inc
    jump 51_while
52_end:
    isrg
    iloadc_1
    jsre 3
    iinc_1 3               ; add -> inc
    jump 49_while
50_end:
    return

printFloatMat:
    esr 2
    iloadc_0
    istore 3
44_while:
    iload_3
    iload_0
    ilt
    branch_f 45_end
    iloadc_0
    istore 4
46_while:
    iload 4
    iload_1
    ilt
    branch_f 47_end
    isrg
    iload_3
    iload_1
    imul
    iload 4
    iadd
    aload_2
    floada
    jsre 1
    iload 4
    iloadc_1
    iadd
    iload_1
    ilt
    branch_f 48_end
    isrg
    iloadc_1
    jsre 2
48_end:
    iinc_1 4               ; add -> inc
    jump 46_while
47_end:
    isrg
    iloadc_1
    jsre 3
    iinc_1 3               ; add -> inc
    jump 44_while
45_end:
    return

scanIntVec:
    esr 1
    iloadc_0
    istore 2
42_while:
    iload_2
    iload_0
    ilt
    branch_f 43_end
    isrg
    jsre 4
    iload_2
    aload_1
    istorea
    iinc_1 2               ; add -> inc
    jump 42_while
43_end:
    return

scanFloatVec:
    esr 1
    iloadc_0
    istore 2
40_while:
    iload_2
    iload_0
    ilt
    branch_f 41_end
    isrg
    jsre 5
    iload_2
    aload_1
    fstorea
    iinc_1 2               ; add -> inc
    jump 40_while
41_end:
    return

scanIntMat:
    esr 2
    iloadc_0
    istore 3
36_while:
    iload_3
    iload_0
    ilt
    branch_f 37_end
    iloadc_0
    istore 4
38_while:
    iload 4
    iload_1
    ilt
    branch_f 39_end
    isrg
    jsre 4
    iload_3
    iload_1
    imul
    iload 4
    iadd
    aload_2
    istorea
    iinc_1 4               ; add -> inc
    jump 38_while
39_end:
    iinc_1 3               ; add -> inc
    jump 36_while
37_end:
    return

scanFloatMat:
    esr 2
    iloadc_0
    istore 3
32_while:
    iload_3
    iload_0
    ilt
    branch_f 33_end
    iloadc_0
    istore 4
34_while:
    iload 4
    iload_1
    ilt
    branch_f 35_end
    isrg
    jsre 5
    iload_3
    iload_1
    imul
    iload 4
    iadd
    aload_2
    fstorea
    iinc_1 4               ; add -> inc
    jump 34_while
35_end:
    iinc_1 3               ; add -> inc
    jump 32_while
33_end:
    return

matMul:
    esr 4
    iloadc_0
    istore 9
26_while:
    iload 9
    iload_0
    ilt
    branch_f 27_end
    iloadc_0
    istore 10
28_while:
    iload 10
    iload 4
    ilt
    branch_f 29_end
    floadc_0
    fstore 12
    iloadc_0
    istore 11
30_while:
    iload 11
    iload_1
    ilt
    branch_f 31_end
    fload 12
    iload 9
    iload_1
    imul
    iload 11
    iadd
    aload_2
    floada
    iload 11
    iload 4
    imul
    iload 10
    iadd
    aload 5
    floada
    fmul
    fadd
    fstore 12
    iinc_1 11              ; add -> inc
    jump 30_while
31_end:
    fload 12
    iload 9
    iload 7
    imul
    iload 10
    iadd
    aload 8
    fstorea
    iinc_1 10              ; add -> inc
    jump 28_while
29_end:
    iinc_1 9               ; add -> inc
    jump 26_while
27_end:
    return

queens:
    esr 3
    iloadc_0
    istore 3
22_while:
    iload_3
    iload_0
    ilt
    branch_f 23_end
    iloadc_0
    istore 4
24_while:
    iload 4
    iload_1
    ilt
    branch_f 25_end
    bloadc_f
    iload_3
    iload_1
    imul
    iload 4
    iadd
    aload_2
    bstorea
    iinc_1 4               ; add -> inc
    jump 24_while
25_end:
    iinc_1 3               ; add -> inc
    jump 22_while
23_end:
    iload_0
    iload_1
    ine
    branch_f 21_end
    return
21_end:
    iload_0
    istore 5
    isrl
    iloadc_0
    jsr 1 __queens_place
    bpop
    return

__queens_isSafe:
    esr 2
    iloadc_0
    istore 2
11_while:
    iload_2
    iload_0
    ilt
    branch_f 12_end
    iload_2
    iloadn 1 1
    imul
    iload_1
    iadd
    aloadn 1 2
    bloada
    branch_f 13_end
    bloadc_f
    breturn
13_end:
    iinc_1 2               ; add -> inc
    jump 11_while
12_end:
    iload_0
    iloadc_1
    isub
    istore 2
    iload_1
    iloadc_1
    isub
    istore 3
6_while:
    iload_2
    iloadc_0
    ige
    branch_f 9_false_expr
    iload_3
    iloadc_0
    ige
    jump 10_end
9_false_expr:
    bloadc_f
10_end:
    branch_f 7_end
    iload_2
    iloadn 1 1
    imul
    iload_3
    iadd
    aloadn 1 2
    bloada
    branch_f 8_end
    bloadc_f
    breturn
8_end:
    idec_1 2               ; sub -> dec
    idec_1 3               ; sub -> dec
    jump 6_while
7_end:
    iload_0
    iloadc_1
    isub
    istore 2
    iload_1
    iloadc_1
    iadd
    istore 3
1_while:
    iload_2
    iloadc_0
    ige
    branch_f 4_false_expr
    iload_3
    iloadn 1 5
    ilt
    jump 5_end
4_false_expr:
    bloadc_f
5_end:
    branch_f 2_end
    iload_2
    iloadn 1 1
    imul
    iload_3
    iadd
    aloadn 1 2
    bloada
    branch_f 3_end
    bloadc_f
    breturn
3_end:
    idec_1 2               ; sub -> dec
    iinc_1 3               ; add -> inc
    jump 1_while
2_end:
    bloadc_t
    breturn

__queens_place:
    esr 2
    iload_0
    iloadn 1 5
    ieq
    branch_f 20_end
    bloadc_t
    breturn
20_end:
    iloadc_0
    istore 1
    bloadc_f
    bstore 2
14_while:
    iload_1
    iloadn 1 5
    ilt
    branch_f 18_false_expr
    bload_2
    bnot
    jump 19_end
18_false_expr:
    bloadc_f
19_end:
    branch_f 15_end
    isr
    iload_0
    iload_1
    jsr 2 __queens_isSafe
    branch_f 16_end
    bloadc_t
    iload_0
    iloadn 1 1
    imul
    iload_1
    iadd
    aloadn 1 2
    bstorea
    isr
    iload_0
    iloadc_1
    iadd
    jsr 1 __queens_place
    bstore 2
    bload_2
    bnot
    branch_f 17_end
    bloadc_f
    iload_0
    iloadn 1 1
    imul
    iload_1
    iadd
    aloadn 1 2
    bstorea
17_end:
16_end:
    iinc_1 1               ; add -> inc
    jump 14_while
15_end:
    bload_2
    breturn

.exportfun "printIntVec" void int int[] printIntVec
.exportfun "printFloatVec" void int float[] printFloatVec
.exportfun "printIntMat" void int int int[] printIntMat
.exportfun "printFloatMat" void int int float[] printFloatMat
.exportfun "scanIntVec" void int int[] scanIntVec
.exportfun "scanFloatVec" void int float[] scanFloatVec
.exportfun "scanIntMat" void int int int[] scanIntMat
.exportfun "scanFloatMat" void int int float[] scanFloatMat
.exportfun "matMul" void int int float[] int int float[] int int float[] matMul
.exportfun "queens" void int int bool[] queens
.importfun "printInt" void int
.importfun "printFloat" void float
.importfun "printSpaces" void int
.importfun "printNewlines" void int
.importfun "scanInt" int
.importfun "scanFloat" float
