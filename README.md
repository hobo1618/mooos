# mooos &#x2014; an OS built in rust

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



