# ZidamOS

## 27-01-2023

### Working with no standard libraries in Rust

To build an operatin system, we can't use standard libraries as these libraries are designed to use resource from a underlying operating system like BIOS or resource management. Hence, the very first thing we must do is to disable the standard libraries using `#[no_std]`. Macros like `println!` or `panic!` will no longer valid.

### Exception handler

Now, the second thing we must do is to handle exception and crash the system if there is an error. As mentioned above, `panic!` is no longer valid. So we must build another function handler to handle panic. However, `stack unwinding` is not supported as the original stack winding of Rust relies on packages on mainstream OS (e.g. libunwind on Linux or structured exception handling on Windows).

#### Stack unwinding

`Stack unwinding` is a very complicated process. In C, we have `setjmp()` and `longjmp()` or in `C++` we have `goto` keyword. The main responsibility of unwinding is to pass the control to another address stored in stack and end the current function's call stack. Every function when is call will allocate a specific space on `stack frame` to store local variables and assign the return address to the register. Hence, when the function is ended, the register value will be popped from the stack for the system to know where to continue.

#### Implementation

Back to panic handler, in Rust, we can use attribute macro `#[panic_handler]` and struct `core::panic::PanicInfo` to build our own panic handler.

```rs
#![no_main]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

This panic handler function is a diverged function in Rust. It means the functions never returns a value and it can only be ended by a machine crash.

### Program entry point

`main()` function is not an entry point of a program. Every programming language has its own runtime system, for example, C has `C zero runtime`. This runtime system acts as a pre-staged program that allocate resource and spaces in memory for the upcoming program before calling the `main()` function. Rust runtime system also relies on the `C zero runtime` or `crt0`. Therefore, we must tell Rust to not use the entry point in its language `#[no_main]` and rebuild the entry point of the program.

`_start()` is also a diverged function as it does not return any value and won't be called by any other function. `start()` will act as an entry point and invoked by a machine directly. Similar to `_start:` in `Assembly`. The `#[no_mangle]` macro is to tell the Rust compiler not to mangle the name of the function. Name mangling is common in compiler design. It mangles the name of function to make every function unique.

```rs
// Tell Rust that we don't use the normal entry points (lang="start") which uses C runtime zero
// Remove main() as we no longer use the normal runtime system that calls main() function
#![no_main]

// Disable name mangling. Make sure Rust compiler truly return _start as entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

```

### Error with linker

After migrating from using standard libraries and Rust builtin panic handler, `cargo build` now will throw error related to `c` linker.

```powershell
 Compiling os v0.1.0 (/Users/chungquantin/zidamos/os)
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-arch" "arm64" "/var/folders/20/yl7z_2g537116dys0rfp6vv40000gn/T/rustcDzymek/symbols.o" "/Users/chungquantin/zidamos/os/target/debug/deps/os-9e22bcc5460d0159.320sn318x4ie2v1n.rcgu.o" "-L" "/Users/chungquantin/zidamos/os/target/debug/deps" "-L" "/Users/chungquantin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib" "/Users/chungquantin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/librustc_std_workspace_core-0251f0b5857602a2.rlib" "/Users/chungquantin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libcore-9382e8c089006a25.rlib" "/Users/chungquantin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib/libcompiler_builtins-6b5d600bff28faab.rlib" "-L" "/Users/chungquantin/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib" "-o" "/Users/chungquantin/zidamos/os/target/debug/deps/os-9e22bcc5460d0159" "-Wl,-dead_strip" "-nodefaultlibs" "-e" "__start" "-static" "-undefined" "dynamic_lookup"
   = note: ld: dynamic main executables must link with libSystem.dylib for architecture arm64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

The problem is, the linker uses the libraries in underlying OS. It supposes that the OS has a C linker (both Windows, Linux and MacOS is built using C).
