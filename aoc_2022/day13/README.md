# Day 13

Comparisons of nested lists.

Have complicated nested lists, such as [[1, 2], 3] and need to compare them
against each other.

Day a: Compare pairs and sum the indices (starting at 1) of all pairs who are in
the correct order left then right

Day b: Add in 2 entries, then sort and multiply the new indices (starting at 1)
of these 2 entries.

the comparison rules are basically lexographic ordering, with one gotcha that
when comparing a list to a literal value, you should compare the list to a
1-sized list of that literal value.

When I submitted my solution to this, the atlassian people asked some questions
about Eq vs Ord vs PartialOrd vs PartialEq that I didn't quite understand, but
have since spent a bit of time learning these, and when I'm not feeling so
covid-ated I might have a go at playing around with some of the ideas there. I
have a feeling that I might only have to define the Literal vs List case and see
what happens if I say std::cmp::Compare for the rest.
