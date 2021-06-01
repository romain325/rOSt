# 006 - CPU Fault

We know some CPU Exceptions, so we're gonna make an interrupt descriptor table (IDT) for the CPU fault

Let's use the same architecture as this:  https://os.phil-opp.com/cpu-exceptions/

As there is quite a lot of exceptions, we're not gonna rewrite the IDT and instead use the x86_64 package one, they look like so:  

```rust
#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
```

So we'll have to write our HandlerFunc -> `type HandlerFunc = extern "x86-interrupt" fn(_: InterruptStackFrame);`  

Following this principle:
![Interrup](https://os.phil-opp.com/cpu-exceptions/exception-stack-frame.svg)

You can have a look at the source code as it is pretty informing: [Source](https://docs.rs/x86_64/0.14.2/src/x86_64/structures/idt.rs.html#40-371)
