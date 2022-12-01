### Day 20

Run a convolution type approach to adjusting images. Each point on an image is lit or unlit, and each 3x3 square is turned into a 9-bit number that references a lookup table of lit or unlit.
We convolve the image based on that index, and lookup the new value. In theory the image is infinite in each direction.

In the example data its easy, as all the infinite squares remain 0 after each loop. In the given data, we have to jump between lit and unlit, and it gets
a bit complicated. The main difference is that we need to give a "is this found" when outside of the current boundary, and return true for all checks on odd
numbered jumps.

#### Part a

Find the lit square count after 2 iterations

#### Part b

Find the lit square count after 50 iterations

#### Notes

I appear to have given up on worrying too much about runtime speed at this point, just trying to get it working. Again I struggled with the gotcha
in this one, so I again found a reddit solution to look at. At first I was just checking that there is an answer that's different to mine, then I played
around with how I was working the boundaries to check until the numbers aligned with what they told me it should be.
