# Badcrypt

Bad and unsafe implementations of crytopals challenges.

# Set 1 - Basics

## 1.1 - Convert hex to base64

This was an interesting experiment in implementing the From trait on a custom
type. Getting a working solution was less complicated than I expected but designing
a reasonably _rusty_ interface was good practice.

I peeked into base64 encoders once I completed this challenge and found that
it was most common to operate over larger word sizes than a single byte. One
example handled 64 bits per iteration to complete the work in 1/4 the cycles.

## 1.2 - XOR Buffers

I kept this simple by iterating through the buffer bytes and executing the xor operation. Encoding each byte to hex is a simple
mapping but I chose to use the **hex** crate to reencode the result.

I may revisit this encoding later.

## 1.3 - Single Byte XOR Cipher