main:
    esr 12
    iloadc 0
    inewa
    astore 0
    iloadc 1
    fnewa
    astore 1
    iloadc 2
    inewa
    astore 2
    iloadc 1
    fnewa
    astore 3
    iloadc 2
    fnewa
    astore 4
    iloadc 2
    fnewa
    astore 5
    iloadc 1
    fnewa
    astore 6
    iloadc 3
    bnewa
    astore 7
    iloadc_1
    iloadc_0
    aload_0
    istorea
    iloadc 4
    iloadc_1
    aload_0
    istorea
    iloadc 5
    iloadc 4
    aload_0
    istorea
    iloadc 1
    iloadc 5
    aload_0
    istorea
    iloadc 0
    iloadc 1
    aload_0
    istorea
    isrg
    iloadc 0
    aload_0
    jsre 6
    floadc_1
    iloadc_0
    aload_1
    fstorea
    floadc 6
    iloadc_1
    aload_1
    fstorea
    floadc 7
    iloadc 4
    aload_1
    fstorea
    floadc 8
    iloadc 5
    aload_1
    fstorea
    isrg
    iloadc 1
    aload_1
    jsre 7
    iloadc_1
    iloadc_0
    aload_2
    istorea
    iloadc 4
    iloadc_1
    aload_2
    istorea
    iloadc 5
    iloadc 4
    aload_2
    istorea
    iloadc 1
    iloadc 5
    aload_2
    istorea
    iloadc 0
    iloadc 1
    aload_2
    istorea
    iloadc 2
    iloadc 0
    aload_2
    istorea
    isrg
    iloadc 4
    iloadc 5
    aload_2
    jsre 8
    floadc_1
    iloadc_0
    aload_3
    fstorea
    floadc 9
    iloadc_1
    aload_3
    fstorea
    floadc 10
    iloadc 4
    aload_3
    fstorea
    floadc 8
    iloadc 5
    aload_3
    fstorea
    isrg
    iloadc 4
    iloadc 4
    aload_3
    jsre 9
    floadc_1
    iloadc_0
    aload 4
    fstorea
    floadc 9
    iloadc_1
    aload 4
    fstorea
    floadc 10
    iloadc 4
    aload 4
    fstorea
    floadc 8
    iloadc 5
    aload 4
    fstorea
    floadc 11
    iloadc 1
    aload 4
    fstorea
    floadc 12
    iloadc 0
    aload 4
    fstorea
    floadc 13
    iloadc_0
    aload 5
    fstorea
    floadc 14
    iloadc_1
    aload 5
    fstorea
    floadc 15
    iloadc 4
    aload 5
    fstorea
    floadc 16
    iloadc 5
    aload 5
    fstorea
    floadc 17
    iloadc 1
    aload 5
    fstorea
    floadc 18
    iloadc 0
    aload 5
    fstorea
    iloadc_0
    istore 8
12_while:
    iload 8
    iloadc 4
    ilt
    branch_f 13_end
    iloadc_0
    istore 9
14_while:
    iload 9
    iloadc 4
    ilt
    branch_f 15_end
    floadc_0
    iload 8
    iloadc 4
    imul
    iload 9
    iadd
    aload 6
    fstorea
    iinc_1 9               ; add -> inc
    jump 14_while
15_end:
    iinc_1 8               ; add -> inc
    jump 12_while
13_end:
    isrg
    iloadc 4
    iloadc 5
    aload 4
    iloadc 5
    iloadc 4
    aload 5
    iloadc 4
    iloadc 4
    aload 6
    jsre 14
    isrg
    iloadc 4
    iloadc 4
    aload 6
    jsre 9
    iloadc_0
    istore 8
8_while:
    iload 8
    iloadc 19
    ilt
    branch_f 9_end
    iloadc_0
    istore 9
10_while:
    iload 9
    iloadc 19
    ilt
    branch_f 11_end
    bloadc_f
    iload 8
    iloadc 19
    imul
    iload 9
    iadd
    aload 7
    bstorea
    iinc_1 9               ; add -> inc
    jump 10_while
11_end:
    iinc_1 8               ; add -> inc
    jump 8_while
9_end:
    isrg
    iloadc 19
    iloadc 19
    aload 7
    jsre 15
    iloadc_0
    istore 10
1_while:
    iload 10
    iloadc 19
    ilt
    branch_f 2_end
    iloadc_0
    istore 11
3_while:
    iload 11
    iloadc 19
    ilt
    branch_f 4_end
    isrg
    iload 10
    iloadc 19
    imul
    iload 11
    iadd
    aload 7
    bloada
    branch_f 6_false_expr
    iloadc_1
    jump 7_end
6_false_expr:
    iloadc_0
7_end:
    jsre 0
    iload 11
    iloadc_1
    iadd
    iloadc 19
    ilt
    branch_f 5_end
    isrg
    iloadc_1
    jsre 4
5_end:
    iinc_1 11              ; add -> inc
    jump 3_while
4_end:
    isrg
    iloadc_1
    jsre 5
    iinc_1 10              ; add -> inc
    jump 1_while
2_end:
    iloadc_0
    ireturn

.const int 5
.const int 4
.const int 6
.const int 64
.const int 2
.const int 3
.const float 2.5
.const float -3.0
.const float 4.0
.const float 2.0
.const float 3.0
.const float 5.0
.const float 6.0
.const float 7.0
.const float 8.0
.const float 9.0
.const float 10.0
.const float 11.0
.const float 12.0
.const int 8
.exportfun "main" int main
.importfun "printInt" void int
.importfun "printFloat" void float
.importfun "scanInt" int
.importfun "scanFloat" float
.importfun "printSpaces" void int
.importfun "printNewlines" void int
.importfun "printIntVec" void int int[]
.importfun "printFloatVec" void int float[]
.importfun "printIntMat" void int int int[]
.importfun "printFloatMat" void int int float[]
.importfun "scanIntVec" void int int[]
.importfun "scanFloatVec" void int float[]
.importfun "scanIntMat" void int int int[]
.importfun "scanFloatMat" void int int float[]
.importfun "matMul" void int int float[] int int float[] int int float[]
.importfun "queens" void int int bool[]
