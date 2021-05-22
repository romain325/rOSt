# 001 - HelloWorld

> Okay I admit it, last on was pretty long, i didn't expect a compilation to be that hard
> Let's do stg easy this time ahah

## tl;dr;

- [x] Boot our kernel
- [x] Display "Hello World!"
- [x] Let's write it in magenta

## Memory

Remember five lines above when i said "stg easy", and now you read "Memory", pretty contradictory isn't it ?  
But no, the memset function is actually pretty decisive and a missed shit in it and everything collapse  
I don't really want to play with it ^^ so let's just import it into .cargo/config.toml
Less scary than creating this function ourselves isn't it !  
Today things will be pleasant, I feel it !

## Booting our kernel

Remember the last line ? This time i lied.  
Time to suffer
As said in the previous topic, we're not gonna have a deep look into booting so let's use bootimage, so add it to the dependencies & run  `cargo install bootimage` (`rustup component add llvm-tools-preview` for the deps)
this tool will allow us to create a bootable img
By running `cargo bootimage` it creates an image with the build we were using earlier (so it keeps our config ^^)  
Sweet I do have a .bin image buuuut how do I use it now ?

## Emulation!

Ok now let's emulate our fresh kernel with QEMU  
install qemu & qemu-system-x86 in order to use the following command: qemu-system-x86_64 -drive format=raw,file=target/x86_64-rOSt/debug/bootimage-rOSt.bin
ok black screen (if you click on it, just use alt+m and navigate to quit), BUT IT WORKS
Okay but i'm lazy so let's add a little config to cargo so it can run this command for me (in the bootimage doc lol)  
Okay so for now on, we'll run `cargo run` in order to get everything running !  

## Hello world!

I said we're gonna print hello world without too much pain so let's try !  
If you have a look at the vga text buffer you can see a special mem address that contains contents displayed on screen, let's use it to display our hello world  
Our VGA buffer is located at `0xb8000` !
We've everything to display our hello world!
Let's write it in our _start method right before our loop{}
And let's make it *fancy* by writing it in magenta

## Resources

- <https://github.com/rust-osdev/bootimage>
- <https://en.wikipedia.org/wiki/VGA_text_mode>
