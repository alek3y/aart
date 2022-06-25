# aart

Short little project for converting an image to a braille ASCII art.

<p align="center">
	<img src="https://i.ibb.co/njs7vzx/image.png" alt="Sailor moon ASCII art" width="640">
</p>

It's probably worth noting the method used to get the braille characters for a set of pixels. \
As seen in the [unicode chart](https://www.unicode.org/charts/PDF/U2800.pdf), each dot incrementally increases the same way binary does.

So, if you consider all of the eight dots as a bit you can create a byte that represents a braille pattern.
For example, the sequence `0b01100011` would represent `⡣`.

This way, to compute the unicode code point you can sum the binary number to `0x2800` which is the first braille character.
