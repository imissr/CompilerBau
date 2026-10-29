main:
    esr 3
    isrg
    jsre 1
    istore 0
    iload_0
    istore 1
    iload_1
    inewa
    astore 2
    isrg
    iload_1
    aload_2
    jsr 2 readValues
    isrg
    iload_1
    aload_2
    jsr 2 printValues
    iloadc_0
    ireturn

readValues:
    esr 2
    iloadc_0
    istore 2
    iload_0
    istore 3
3_while:
    iload_2
    iload_3
    ilt
    branch_f 4_end
    isrg
    jsre 1
    iload_2
    aload_1
    istorea
    iinc_1 2
    jump 3_while
4_end:
    return

printValues:
    esr 2
    iloadc_0
    istore 2
    iload_0
    istore 3
1_while:
    iload_2
    iload_3
    ilt
    branch_f 2_end
    isrg
    iload_2
    aload_1
    iloada
    jsre 0
    iinc_1 2
    jump 1_while
2_end:
    return

.exportfun "main" int main
.importfun "printInt" void int
.importfun "scanInt" int
