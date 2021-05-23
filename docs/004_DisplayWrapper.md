# 004 - Display Wrapper

Okay so we can print and all that fun but let's make it a bit more versatile !!  
Rn we need to init a writer each time and that sucks, let's make it static (maybe not ;) ) and let's implement the default rust function to it too !  

## tl;dr;

- [x] String formatting  
- [x] Access from everywhere  
- [x] Accessible AND thread secure  
- [x] Finally have a panic display  
- [x] Change color programmatically
- [x] Print panic in read

## rust str macros

Okay you know templating like `write!("test: {}", 666)`, let's implement our writer to it so we can use it !  
And it works fine with char and everything but whenever i compile it with nb as apram, it panics, aaannnd i figured out that my memory system is shit (who would have known !)
Let's apply some correction to the memory sys !  
After a quick look at https://github.com/rust-lang/compiler-builtins/blob/master/src/mem/mod.rs
I conclude that the code is exactly the same but cleaner ! I'll keep my code as a PoC but will use the builtins mem for now on

## Static ? not really  

Okay we implemented this wonderful macro, let's now add a function allowing us to print from wherever we want without having to instantiate our VGAWriter  
Ok but just using static doesn't work at all and it seems pretty logic, we're creating new color and casting a pointer, illegal for static !  
We could use workaround buuut it would look shitty so let's use lazy_static! add it to the cargo.toml file
Remember to add the no std in order to continue this bare metal  
Now we need to add mutability to print things in it  

## Static mutability ? not really

Once again it isn't a good idea to use mutability over a static element, so we'll avoid it !  
Let's instead use a mutex ^^, it's contained in the spin crate
And  in our main we can get rid of all ! Let's just use our writer, lock it and write what you want !  

## Our own macro

With our own macro we could do a println ! would be amazing !  
Thx god rust is open source so we can always have a look at what already exists in the std

## Time for some bonus !

U know what i miss ? having a display on panicking ?  
Imagine panicking and seeing your scream for help going in the deep, that's horrible, poor rusty rust !  
What we'll do is rewriting our panic handler to print the PanicInfo with our macro !!  

## Resources

- <https://github.com/rust-lang/compiler-builtins/blob/master/src/mem/mod.rs>
