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

#### SATOR mode

The SATOR square is a 5x5 magic square written mostly in Latin, with one unknown word:

```
S A T O R
A R E P O
T E N E T
O P E R A
R O T A S
```

```sh
cargo run -- ./scrabble-common.lt6.txt ____ 5 SATOR
```

When you run in SATOR mode, you will see guesses populated in palindromic sequence:

```
a b l e
w _ _ _
_ _ _ _
_ _ _ _
_ _ _ w
e l b a
```

Note that at least in the standard English Scrabble dictionary, there are no 5×5 SATOR squares.

Some other shapes, such as 4×4, are indeed found (these were the first squares contributed to the Wikipedia article on the SATOR square :))

(I don't like this one, but it's a valid "Scrabble SATOR Square" because an "anna" is a former unit of currency in Burma, India, and Pakistan.)

```
a n n a
n o o n
n o o n
a n n a
```

This one is cool but doesn't have a nice phrasal interpretation:

```
p e t s
e d i t
t i d e
s t e p
```

If you find a new SATOR square, please share!

## About the algorithm

The algorithm implemented here is a basic backtracking search. It is not optimized for speed, but it is capable of finding solutions for small dictionaries and small rectangles. The algorithm is not guaranteed to find a solution for all inputs.

It starts by adding a letter at the next unspecified position and checking for validity. Validity is defined by all possible spans in vertical/horizontal directions having at least one valid template match in the dictionary. If the current state is invalid, the algorithm backtracks to the previous state and tries a different letter.

## Roadmap

-   [ ] Toggleably reorder the alphabet to prevent always returning the lexicographically first solution
-   [ ] Multithread the search, with cancellation of threads when an earlier thread finds a solution
