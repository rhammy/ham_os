# Ham OS (Rust)

## Chapter 1: A Baremetal Rust Binary

I am going to write an OS in Rust. Each chapter will include my thoughts, notes, and important details.

First I need to create a baremetal binary of rust so it can run without the standard library (which uses system calls that aren't available). A typical Rust binary that links with the standard library starts execution in a C runtime library `crt0` (this creates the stack and palces args in the right registers). This baremetal executable does not have access to the Rust start point for runtime or `crt0` so I need to define an entry point for the runtime system.

`rustc --version --verbose` displays the host target system and what rust is attempting to compile against. We don't want to compile for the host system so we will use an embedded ARM chip target (`thumbv7em-none-eabihf`) to simulate compiling for a target with no OS.
This will cross-compile our executable for a bare metal target system!

```
 rustup target add thumbv7em-none-eabihf
 cargo build --target thumbv7em-none-eabihf
```

## Chapter 2: A Mini Kernel

Now to build a bootable disk image that prints to the screen. When you turn on a box, it loads the BIOS from flash storage on the motherboard. The BIOS looks for bootable disks, finds one, then control is tranferred to a 512-byte section of executable code stored at the disk's beginning called the _bootloader_.

The bootloader finds the kernel image on the disk and loads it into main memory, tranistions the CPU into 64-bit mode if it was in 16-bit mode to accommadate the BIOS, then copies the memory map from the BIOS into the kernel. I will not be building a bootloader from scratch. Intead I will automagically prepend a bootloader to the beginning of my kernel image.

Thank God for the Multiboot standard and Grand Unified Bootloader (GRUB). Add a multiboot-header at the beginning of kernel file to be multiboot compliant.
Now adding a target spec json file `x86_64-ham_os.json` to define the target system we are compiling the kernel for.

Now we do some unsafe operations on the VGA buffer to print a string to the console. I needed to use mobaXterm in order for a QEMU window to launch because VSCode does not support this.

## Chapter 3: VGA text but safer



## Sources

https://os.phil-opp.com/freestanding-rust-binary/