# maybe
A small, statically-typed toy language with a tree-walking interpreter written in Rust (no dependencies).
Source files use the `.mbl` extension.

```
var int i = 0;
var list[int] arr = [];
loop {
    match i < 10 {
        true {
            arr.push(i);
            print(arr[i]);
            i = i + 1;
        }
        _ { break }
    }
}
```

## Building and running

```sh
cargo build --release
cargo run -- run test.mbl
```

```
maybe <command> <file.mbl>
```

| Command       | What it does                                                     |
|---------------|------------------------------------------------------------------|
| `run`         | Resolve and execute the program                                  |
| `tokens`      | Print the token stream                                           |
| `statements`  | Print the parsed AST                                             |
| `rstatements` | Print the AST after name resolution (variables replaced by ids)  |

## Pipeline

`tokenizer.rs` → `parser.rs` (recursive descent) → `resolver.rs` (maps variable names to slot ids, checks scopes) → `runtime.rs` (evaluates the resolved AST). Shared types live in `types.rs`.

## Language reference

### Comments

```
// line comments only
```

### Types and literals

| Type        | Literal examples       | Notes                                   |
|-------------|------------------------|-----------------------------------------|
| `int`       | `0`, `42`              | 32-bit signed; no negative literals yet |
| `bool`      | `true`, `false`        |                                         |
| `char`      | `'a'`                  | single character                        |
| `str`       | `"hello"`              | no escape sequences                     |
| `list[T]`   | `[]`, `[1, 2, 3]`      | nestable: `list[list[int]]`             |
| —           | `null`                 | assignable to a variable of any type    |

### Variables

```
var int x = 1;      // declaration: type is required, initializer is required
x = x + 1;          // assignment: value must match the declared type
```

- Identifiers consist of letters only (no digits or underscores).
- Type checks happen at runtime, on declaration and on assignment.
- Variables are block-scoped (`loop` and `match` arm bodies open a new scope).
- Shadowing is not allowed: redeclaring a name that is visible in an enclosing scope is an error.

### Operators

| Operator         | Operands                                                        |
|------------------|-----------------------------------------------------------------|
| `+ - * /`        | `int` (integer division; `/ 0` is a runtime error)              |
| `+`              | concatenation of any mix of `str`, `char` and `int` → `str`     |
| `==  !=`         | any values (structural equality)                                |
| `<  >`           | `int` only                                                      |

Precedence (high to low): indexing / method calls, `* /`, `+ -`, comparisons. Parentheses group.
Adding `null` to an `int`, `str` or `char` returns the other operand unchanged.

### Built-in functions

- `print(expr);` — prints a single value followed by a newline.

### `match`

`match` is the only branching construct. Arms are tried in order; the first matching arm runs.

```
match x {
    5      { print("exactly five"); }   // literal / expression: subject == value
    < 3    { print("small"); }          // half-binary: subject < 3
    > 100  { print("big"); }            // also == and !=
    _      { print("other"); }          // wildcard, must be last, at most one
}
```

An `if`/`else` is written as `match cond { true { ... } _ { ... } }`.

### `loop`, `break`, `continue`

```
loop {
    i = i + 1;
    match i {
        == 2 { continue }
        > 5  { break }
        _    {}
    }
    print(i);
}
```

`loop` is an infinite loop. `break` and `continue` are written **without** a trailing `;`.
They propagate out of `match` arms to the enclosing loop; `break` outside a loop is a runtime error.

### Lists

```
var list[int] a = [1, 2];
a.push(3);          // append (exactly one argument)
print(a.len());     // 3
print(a[0]);        // 1
a[0] = 10;          // index assignment
```

- Out-of-bounds reads and writes are runtime errors.
- Lists are reference values: `var list[int] b = a;` makes `b` and `a` share the same list.

## Errors

- **Resolution errors** (before execution): undefined variable, variable already defined.
- **Runtime errors**: type mismatch, division by zero, invalid operand types, index out of bounds,
  unknown function or method, `break` outside a loop.
- **Syntax errors** currently abort with a Rust panic.

## Known limitations

- `.push()` does not check the element type (`list[int]` can receive a `str`).
- `print` on a list prints its internal heap index instead of its contents.
- An index on the right-hand side of `*` or `/` fails to parse (`2 * a[0]`); write `a[0] * 2` instead.
- No user-defined functions yet; `print` is the only function.
