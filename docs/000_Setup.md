# 000 - Setup

## tl;dr;

- [x] Compile without std lib  

---

## Create the project

We need to first create the project:

`cargo new rOSt --bin`

We'll use rust nightly so in the current folder run `rustup override set nightly`  

And in the main.rs file we add  

```rust
#![no_std]
```

in order to avoid the default std function (let's to this bare metal)  
Empty the main function and run `cargo build`

## Error ?

even with the main function empty, the compiler is driving crazy! *#[panic_handler] function required but not found*  
The std contains a panic handler triggered on panic (obvious ^^), we need to rewrite one!  
For now we can't really do anything so let's just implement it empty, we'll fulfill it later.  

## Errors ? Still ?

Ok but what about the eh_personality error ?  
As mentioned in the doc add profiles in your Cargo.toml (panic = "abort" for both dev and release)  
By doing this you're disabling unwinding

## Will it ever compile ?

No.  
Ok a bit pessimistic on this one, but "start" is driving crazy too.  
Before starting our main, we need to define a stack and a heap right ? How do u think it's done ? AH ! you don't know ! me neither tbh  

First let's replace our empty main by `#![no-main]` as it doesn't make any sense to have a main without anything behind it

Briefly explained, the rust first call a C program (crt0) which is the entrypoint of everything, it allocates memory and then start the rust program with the start function
Let's rewrite this with an empty loop again and the no mangle so that rust doesn't overwrite the name

And now we get some ugly error messages ! linker errors in particular
Quickly explained, rust does detect your archi and link what is supposed to be under it, but rn it's going crazy so we need to tell him to start bare metal (the target triple thumbv7em-none-eabihf will do the job perfectly for now, just run `rustup target add thumbv7em-none-eabihf` and then cargo build with --target set to thumbv7em-none-eabihf)

> Well done! the tl;dr; was to compile without errors ! you did it !
> But not really yet :/

## Bootloader

We've seen a lot and now we're facing the bootloading in order to start on a x86_x64 computer, aaaaand no, no, no i'll not get through this as I don't want to play with registry rn, I'll try one day, but not today
Let's use bootimage, a rust tool creating it for us ^^

## Back on our target

K. for the bootloader but we didn't provide a solution for our target, we want to run a compilation on top of stg that doesn't have anything configured before hand ?  
Yep and Rust is amazing bc we can do that from a json file lol  
We create a x86_64-rOSt.json file and fill it
We can now run cargo build with our json file as target

## One more error and I give up

First of all: pls don't give up now
Secondly: Yes you get told that "core" doesn't exist, bc it is true, in the context of your newly created target, it is true  
Here comes the handy "nightly" feature, which allows you to build core sources for a specified target
So let's add a few lines in .cargo/config.toml and go ahead !

## Well done !

Yeah !!  It compiles !  
Does nothing but compile !!  
Next time we'll dig a bit and start to play with some memory !  

## Resources

- <https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html>
- <https://os.phil-opp.com/freestanding-rust-binary/>
