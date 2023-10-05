# mooos&#x2014;an OS built in rust

notes on [Phil Oppermans's blog](https://os.phil-opp.com)

## 1 Bare bones

### 2 A Freestanding Rust Binary

[link to page](https://os.phil-opp.com/freestanding-rust-binary/)

#### What is *eh_personality* ?

This is a "language item", like the "Copy" trait, which "marks a function that is used for implementing [stack unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php)."
A personality function attribute ["permits functions to specify what function to use for exception handling."](https://llvm.org/docs/LangRef.html#id1760).
Presumably when an exception occurs, then, the *eh_personality* trait tells LLVM how to unwind the stack, i.e. "to run the destructors of all live stack variables in case of a panic....\[which] ensures that all used memory is freed and allows the parent thread to catch the panic and continue execution." 
Here's how it's described in the [docs](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/personality/gcc.rs):

> Implementation of panics backed by libgcc/libunwind (in some form).
> 
> For background on exception handling and stack unwinding please see
> "Exception Handling in LLVM" (llvm.org/docs/ExceptionHandling.html) and
> documents linked from it.
> These are also good reads:
>  * <https://itanium-cxx-abi.github.io/cxx-abi/abi-eh.html>
>  * <https://monoinfinito.wordpress.com/series/exception-handling-in-c/>
>  * <https://www.airs.com/blog/index.php?s=exception+frames>
> 
> __A brief summary__
> 
> Exception handling happens in two phases: a search phase and a cleanup
> phase.
> 
> In both phases the unwinder walks stack frames from top to bottom using
> information from the stack frame unwind sections of the current process's
> modules ("module" here refers to an OS module, i.e., an executable or a
> dynamic library).
> 
> For each stack frame, it invokes the associated "personality routine", whose
> address is also stored in the unwind info section.
> 
> In the search phase, the job of a personality routine is to examine
> exception object being thrown, and to decide whether it should be caught at
> that stack frame. Once the handler frame has been identified, cleanup phase
> begins.
> 
> In the cleanup phase, the unwinder invokes each personality routine again.
> This time it decides which (if any) cleanup code needs to be run for
> the current stack frame. If so, the control is transferred to a special
> branch in the function body, the "landing pad", which invokes destructors,
> frees memory, etc. At the end of the landing pad, control is transferred
> back to the unwinder and unwinding resumes.
> 
> Once stack has been unwound down to the handler frame level, unwinding stops
> and the last personality routine transfers control to the catch block.

#### the linker, the `C` calling convention, and `ABI`s

When we mark our `_start`ing entry point as `extern "C"` we're effectively tricking the compiler into using the [C calling convention](https://en.wikipedia.org/wiki/Calling_convention), even though we're not actually calling the C runtime.
Presumably the linker is fooled too, thinking our code depends on the C runtime, and when it can't resolve the dependency, it throws an error.

```bash
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" […]
  = note: /usr/lib/gcc/../x86_64-linux-gnu/Scrt1.o: In function `_start':
          (.text+0x12): undefined reference to `__libc_csu_fini'
          /usr/lib/gcc/../x86_64-linux-gnu/Scrt1.o: In function `_start':
          (.text+0x19): undefined reference to `__libc_csu_init'
          /usr/lib/gcc/../x86_64-linux-gnu/Scrt1.o: In function `_start':
          (.text+0x25): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status
```

> A linker is a programming tool which combines one or more partial Object Files and libraries into a (more) complete executable object file. .... It will also walk the "missing items" list and check other object file's symbol table to make sure every dependency can be resolve. Be it a single printf symbol that cannot be found anywhere, the linker aborts here and throw you an error message. 
[source](https://wiki.osdev.org/Linkers)


##### 1. To avoid the error, we can change the target. A target triple takes the form:

\[cpu-architecture]-\[vendor]-\[operating-system]-\[ABI]

__Examples:__

OS      | target
---     | ---
linux   | `x86_64-intel-linux-gnu`
linux   | `x86_64-unknown-linux-gnu`
windows | `x86_64-pc-windows-msvc`
none    | `thumbv7em-none-eabihf`
none    | `x86_64-unknown-none`


2. We want a target without an underlying OS such as `thumbv7em-none-eabihf` or `x86_64-unknown-none`. Cross compiling to either of these targets would signal to the linker that we don't depend on the C runtime, and the error would go away. Poof!

##### 2. We can also solve the linker error by adding build commands to a `.cargo/config.toml` file.

```rust
# in .cargo/config.toml

[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]

[target.'cfg(target_os = "windows")']
rustflags = ["-C", "link-args=/ENTRY:_start /SUBSYSTEM:console"]

[target.'cfg(target_os = "macos")']
rustflags = ["-C", "link-args=-e __start -static -nostartfiles"]
```

If we run `cargo rustc` now, we'll pass different arguments to the compiler depending on the target os. The above `rustflags`, in other words, are appended automatically as follows:

linux: `cargo rustc -- -C link-arg="-nostartfiles"`
windows: `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`
macOS: `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`
















