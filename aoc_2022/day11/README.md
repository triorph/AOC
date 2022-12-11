# Day 11

I did the majority of this at my parents while having dinner. Thankfully mum and
lulu were having intense conversations that didn't give me much time to speak,
so I wasn't missed lol.

I feel like this was mostly straight forward. Part a failed for because I
assumed each monkey was acting in parallel, so monkey 4 wouldn't process monkey
2's bananas from the same round. Once I sorted this it just worked.

Part b needed some modular arithmetic. I was having trouble thinking well (been
pretty tired these last few days, which I'm hoping to remedy soon, although
these 6pm start AOC problems are definitely not helping), but I semi-quickly
realised you need a "shared modulo" between all the monkeys, which I'd just set
as the product of them all. That passed for the test data, but failed for my
input. So then I spent some time adding Least Common Multiple (and also greatest
common denominator) to my shared library, but things were still failing so I
changed those again assuming I'd written it wrong, but then added some checks on
the input modulos on the final result (and eventually realised that the least
common multiple was just the product anyway). In frustration I start writing a
unit test to check that the input data is correct and that the output isn't the
failed answer I keep supplying, only for the unit test to agree that the output
isn't the failed answer. WTF. Turns out I was using the data that had been
poisoned by day a for day b. FML.
