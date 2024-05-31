Minimal x86_64 seL4 root task (in rust!)
====================
I felt like information about seL4 with rust is sparse at best and exploration is hard, so I decided to make this.
I tried to make it as straightforward to compile and modify as I could, so you can play around and get a feel for rust+seL4.

Setup & Build
---------------
```
git clone https://github.com/MelonenBiber/sel4-rust-example-x86_64 --recursive
cd sel4-rust-example-x86_64
./build.sh
```

How to Run
---------------
```
qemu-system-x86_64 \
-cpu Nehalem,+pdpe1gb,+fsgsbase \
--nographic \
-m 128M \
-kernel target/out/kernel32.elf \
-initrd target/out/sel4-rust-example-x86_64.elf
```
alternatively on any modern cpu with virtulization support enabled you can use
```
qemu-system-x86_64 \
-accel kvm \
-cpu host \
--nographic \
-m 128M \
-kernel target/out/kernel32.elf \
-initrd target/out/sel4-rust-example-x86_64.elf
```
