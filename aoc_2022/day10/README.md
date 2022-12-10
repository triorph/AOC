# Day 10

A bit messy to write, but mostly fine. Had a lot of trouble understanding the
part b algorithm from the writing in the examples. This might have just been
because I'm feeling pretty tired today but I wrote some code for a completely
different algorithm before reading it a tenth time and finally figuring out what
it meant. This was the example this year where it was more of a pain to get the
answer as there's an element of OCR, and writing the tests for similar behaviour
is a real pain. In the end I was able to get there on both points after about 1
hour, which isn't too bad given how tired I'm feeling today.

One thing this one did show me is that I was naive in assuming that the data
type returned for part a and part b are the same. At first I refactored the code
to handle dual data types, but then I realised that `calculate_day_a` and
`calculate_day_b` are private methods and shouldn't be defined in the trait
anyway, as a) I might not even want them, and b) I may need to change the
signature to use `&mut self`, instead of `&self`.
