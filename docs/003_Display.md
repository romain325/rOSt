# 003 - Display

Remember our Hello world ? it was pretty fun and stuff but the hello world is hard coded and if you want to change the color or anything we need to change everything and shit  
You guessed it we're gonna make function allowing use to display to the VGA buffer !  

## tl;dr;

- [x] Hello World but programmatically
- [x] Pretty this print !  
- [x] Play around with fg & bg color

## Before we start anything

Okkk in order to pretty up things, we're gonna make a kernel subfolder containing our mem gestion and the vga_buffer file too !  
I like to keep main alone  

## Colors

Ok let's start by setting up our colors ! Let's create an enum of u8 to represent colors and create a ColorCode structure representing both the fg & bg color (as we only need a u4 for one color, let's do bitwise operation)

## Text buff

Okay now that we have our color mapping/struct so let's just create a char buffer and a complete buffer allowing us to write to our buff
Our CharBuffer contains a few informations: char/color/isBlinking
To limit buffer size, we use wikipedia to get VGA buffer width(80) and height(25)

## Writer  

Now we need to implement a Writer that is able to write our chars to the buffer
It contains a fn writing the bytes to our display buuuut first problem, how do I transpile my '\n' to a new line ?

### New Line

We need to first fill the row with empty char (clear_row) and then going to the next line and setting the col pos to  0  

### Actually writing it

Okay now we can make the function writing a single byte, iterate through a string and write each byte to write a str !  

## Where is the problem ?

Okay you can run it and it work fine, that's cool. So why would you think there is a problem ? but there is one, rust is known to have a pretty agressive optimisation on compilation and maybe with another compiler, the fact that you write and never read again from your buffer make it useless and it can be dereferenced.  
So in C++ i would add the "volatile" keyword, and in rust in will use the volatile struct!  

### Volatile

Add volatile to the deps and wrap your VGABufferChar with Volatile  
Okay, now i'll need to update the assignation and read with the method in it (write() & read())  
