# Regular Expressions (REGEX)

Regex is a useful tool that `settle` has support for, because it provides
wildcards and patterns which allow matching multiple strings. See [this regular
expression specification](http://www.math.clemson.edu/~warner/M865/RegexBasics.html) for
all supported patterns. But here are a few of the most useful characters you're
going to use:

- `.` - match any single character
- `*` - match the previous character zero or more times
- `+` - match the previous character one or more times

If you wanted to match a literal `.`, `*` or `+`, you'd have to escape them with
a backslash: `\.`, `\*` and `\+` respectively.

Here are some examples:

- `.*` matches *anything*, even empty strings
- `f+` matches the character `f` one or more times
- `.*foo.*` matches any string containing the word `foo`
