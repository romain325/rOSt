# 002 - Memory

Hello there, has a great night ?  
Yesterday we imported the memory function in order to be able to run our Hello world, lotta fun right ?  
Okay now it's time to implement our memory function by ourselves

In all POSIX we have a function set for mem that are: memset, memcpy, memmove, memcmp
Let's get through each and everyone of them

As it is kinda hard, let's get a bit of inspiration from redox (I do really like this OS)
Oh and btw, the code will start to get a bit complex so if the RUST syntax or logic so take a crashcourse before lol  
I'll not be really greedy in explanation as it is just iterating through data, writing const on top of a given space  

## tl;dr;

- [x] allocate memory ourselves  
- [x] Get rid of the builtins-mem  
- [x] Move, Copy, Set & Compare memory  

## memset

So let's first reinterpret a bit of memory from a type to another
The implementation is from REDOX and is faster as it implement space by 8bytes block and once at the end, iterate 1by1 to the end of the buff size  

## memcpy

Okay now we have the capability to set data, let's copy some from a point to another  
Let's use the same technique of 8bytes blocks for a faster copy and just iterate through both at the same time to copy  

## memmove

Let's move our data now !  
It'll certainly be the hardest of our func set (not too hard tho, it's just one more if) ! And oc we'll use the same allocation technique for a fast af memory set!  

## memcmp

And for our last one let's just compare two memory block !  
As always we'll use the implementation where we first check 8bytes block and if it's different, locate precisely where it differs by iterating through it  

## Conclusion  

Okay maybe it wasn't that long to read but it was a bit more to think about the implementation and code but it wasn't hard tbh as it's juste iterating through memory ahah!  
The only thing you really need to take care of is the type you give to your memory block.  
Of course, to see the result of your amazing memory system you need to remove the builtins-mem in your .cargo/config.toml !  

## Resources

- <https://doc.redox-os.org/kernel/kernel/externs/fn.memset.html>
- <https://doc.redox-os.org/kernel/kernel/externs/fn.memset.html>
