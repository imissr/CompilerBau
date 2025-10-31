main:
    isrg
    iloadc 0
    jsre 6
    isrg
    iloadc 0
    jsre 7
    iloadc_0
    ireturn

.const int 10
.exportfun "main" int main
.importfun "printInt" void int
.importfun "printFloat" void float
.importfun "scanInt" int
.importfun "scanFloat" float
.importfun "printSpaces" void int
.importfun "printNewlines" void int
.importfun "fibs" void int
.importfun "primes" void int
