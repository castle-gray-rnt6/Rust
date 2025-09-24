# Walkthrough our code

- TODO: Add TOC with https://github.com/jonschlinkert/markdown-toc
- TODO: Search for all instances of "TODO" and eliminate them
- TODO: Rename `loop_over_roots` to `skip_cyclotomic_integers` or `filter_cyclotomic_integers`? Change this README accordingly

## Rust

### Run our code

[Rust](https://www.rust-lang.org/) is a programming language made by Mozilla
targeting safety and speed. We use it for the heavy computations of our
project.

Contrary to SageMath or Python, Rust is a _compiled language_. This means that
to run our code, one first needs to _compile_ the source code. Assuming Rust
[is already installed on your
machine](https://www.rust-lang.org/tools/install), instructions are as follows:

```bash
git clone TODO REPO_URL
cd TODO 
cargo build --release
cd target/release  # Go to the directory of the executable
./cassels          # Run our program
```

### Explanation of the `rust` folder

We have three main files:

- In `cyclotomic.rs` we create a _struct_ `CyclotomicInteger` to represent our
cyclotomic integers. Its only method of interest is `castle_strictly_less`,
which asserts whether or not the castle of the cyclotomic integer is strictly
less or not than an input cutoff. 
- In `cassels.rs` we use the public functions from `cyclotomic.rs` to create a
function `loop_over_roots`, responsible for filtering out cyclotomic integers
that are classified under item (1), (2) or (3) of Cassels' theorem (theorem 1.1
in our paper).
- In `main.rs`, we call `loop_over_roots` over an explicit list of cyclotomic
integers. Among those, we obtain candidates that are not covered under the
three aforementioned items.

After that, results are piped to SageMath for exact computations.

## SageMath

TODO: Can somebody write this part, as the continuity of the previous part?
