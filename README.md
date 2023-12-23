# fastrs

_Fastrs_ is a Python extension written in Rust using PyO3 providing a fast and efficient method to
join strings in Python using Rust's performant implementation.

## Installation

```sh
pip install fastrs
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
