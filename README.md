# Novis

Novis is a simple, tree-walking interpreter written in Rust.
Currently under development.

## Usage

```bash
cargo run --release -- <file>
```

or

```bash
cargo run --release -- <file> --bench
```

if you want to measure the execution time.

## Examples

### Hello World

```lua
print "Hello, World!";
```

### Factorial

```lua
func factorial(n) -> {
    if n <= 0 {
        return 1;
    }

    return n * factorial(n - 1);
}

-- Let's test it!
print factorial(5);
```
