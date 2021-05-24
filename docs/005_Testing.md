# Testing

As you may know, testing is primordial, and Rust is a language well known to be as secure as possible soooo testing is a huge part of it.  
Implementing tests in our kernel is mandatory to continue in a healthy environnement (and it can help us benchmark our OS lol)  
Rust has some pretty amazing tool to work with in his core and we'll make them available ^^  
But the test crate make uses of std sooo we'll need to make sure the gate between testing and our kernel is solid!  

## Stack unwinding ?

Do you remember a bit earlier ? we deactivated stack unwinding, and some parts of the test uses stacck unwinding to work, it is a barely simple example of why just creating a gate between test and our kernel isn't a good idea, instead we're gonna implement a custom test set.  
It'll rely on what we need  

## Test mod

The code isn't really hard, I implemented a tester capable of running test and printing out the result to stdout we used previously.  
