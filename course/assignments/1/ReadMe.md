Beim Ausführen erwartet die CiviC-VM genau eine exportierte Funktion `main`
ohne Parameter und mit dem Rückgabetyp `void`:

```c
export void main() {
    return;
}
```

`int main()` wird von der VM nicht als Einstiegspunkt akzeptiert. Für Programme,
die mit der CiviC-VM ausgeführt werden sollen, muss daher `export void main()`
verwendet werden.
