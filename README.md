# rust-automate

This is a simple automate written in Rust that takes a grammar file and an input file and outputs the result if the input is accepted by the grammar.

## Usage

In order to use this automate, you need to have Rust installed on your machine. You can install it from [here](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/iverly/rust-automate
cd rust-automate
cargo run --release -- -g <grammar_file> -i <input_file>
```

To build the project, you can use the following command:

```bash
cargo build --release
```

> The binary will be located in the `target/release` folder.

> The `--release` flag is optional, but it is recommended to use it for better performance.

### Grammar file

You can find some examples of a grammar file in the `examples` folder.

The grammar file must be a json file and follow the format located in the `src/grammar.rs` file.

### Input file

The input file must be a text file containing the input for the automate.

## Examples

To run the examples, you can use the following command:

```bash
cargo run -- -g example/complex-correct/grammar.json -i example/complex-correct/input
```

> Replace `complex-correct` with the example you want to run.

> You can also use the `--release` flag for better performance.

This will run the automate with the `complex-correct` example. The output should be:

```bash
📚 Grammar to use:

S -> C
C -> contact identifier identifier number number | contact identifier identifier number number D | contact identifier identifier number number R | None
R -> rate number number number R | rate number number number D | rate number number number C
D -> delay number number number R | delay number number number D | delay number number number C

📝 Input to be analyzed:

contact A B 20 32
rate 1 10 3
rate 5 1 26
delay 3 50 300

contact T A 10 3
delay 1 5 20

contact Y U 5 16

✅ The input is correct
```

# Contributing

Contributions are welcome. Please follow the standard Git workflow - fork, branch, and pull request.

# License

This project is licensed under the Apache 2.0 - see the `LICENSE` file for details.
