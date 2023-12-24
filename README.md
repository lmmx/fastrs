# fastrs

_Fastrs_ is a Python extension written in Rust using PyO3 providing a fast and efficient method to
join strings in Python using Rust's performant implementation.

Note: upon testing, it was actually slower than Python. Repo left here for future reference (my first Rust extension!).

A simple benchmark shows Python stdlib 3x faster than this code. It's possible other string types
may provide faster alternatives but I've not tried them yet.

## Installation

```sh
maturin develop --release
```

## Usage

```py
import fastrs

result = fastrs.join_strings(["Hello", "Python", "world", "from", "Rust"], separator=" ")
print(result) # Output: "Hello Python world from Rust"
```

If no separator is provided, the strings are simply concatenated.

```py
import fastrs

result = fastrs.join_strings(["Hello", "World"])
print(result) # Output: "HelloWorld"
```

## Benchmarking

- See [gist](https://gist.github.com/lmmx/d1d55af1da500916a66319b6aecdd50b), ~4x slower
