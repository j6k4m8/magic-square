# magic squares

Magic squares are matrices of letters that form valid words when reading across as well as down. You can think of them as a crossword puzzle solution with no blank squares.

```
H E L P
O V A L
M E N U
E N D S
```

This code generates magic rectangles of words from a dictionary file. It also has a few useful ways of invoking the script to "steer" the search for a valid solution.


## Usage

#### Generate the (lexicographically first) magic rectangle from a dictionary file

```sh
cargo run -- wordlist.txt
```

#### Generate a magic rectangle with a specific word in the first row

```sh
cargo run -- wordlist.txt "hello"
```

```
h e l l o
o c e a n
s h a k e
t o n e s
```

#### Generate a magic rectangle with three rows and the second row is "puppy"

Note that there are five underscores before the slash, to indicate five letters unspecified by this template. The slash character indicates a new line. Word lengths are guessed by the number of letters in the template.

```sh
cargo run -- ./scrabble-common.lt6.txt _____/puppy 3
```

#### Generate a magic rectangle with the vertical word "fun" in the second column

```sh
cargo run -- ./scrabble-common.lt6.txt _f_/_u_/_n_ 3
```

```
a f f
c u e
e n d
```

## About the algorithm

The algorithm implemented here is a basic backtracking search. It is not optimized for speed, but it is capable of finding solutions for small dictionaries and small rectangles. The algorithm is not guaranteed to find a solution for all inputs.

It starts by adding a letter at the next unspecified position and checking for validity. Validity is defined by all possible spans in vertical/horizontal directions having at least one valid template match in the dictionary. If the current state is invalid, the algorithm backtracks to the previous state and tries a different letter.

## Roadmap

- [ ] Toggleably reorder the alphabet to prevent always returning the lexicographically first solution
- [ ] Multithread the search, with cancellation of threads when an earlier thread finds a solution
