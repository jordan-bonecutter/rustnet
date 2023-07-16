# rustnet
Feed Forward Neural Network with no dynamic memory allocations (no malloc)

## Why?

I want to see how far I can take rust const generics. This is not practical to use in this way as the size of 
the architecture must be known at compile time (meaning no reading in/downloading networks) but is a fun test to see how far const generics can go.
