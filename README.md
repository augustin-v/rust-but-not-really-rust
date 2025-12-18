C written in Rust experiment.
no_std no_main



```
make main.o # machine code generation step
make executable # linking step
```
or simply:
```
make all
```
for the 2 steps at once.

in case of any error consider buying an old intel mac.

Goal:

write a custom malloc implementation in C (rust), using `sbrk`. Parse CLI arguments through `argv`.
Only use pointers