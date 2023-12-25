<h1 align="center">musical specs</h1>

# Primitives
### Comments
Single lines comments start with a `#`. For example:
`#!/usr/bin/musical` is a valid comment.

Multiline comments start with a `<` and end with a `>`. Implementing nested multiline comments is not necessary nor adviced.

### Note
Syntax:
```
<Note name><Modifier> (notice no whitespace)
```

Note names: one of `A`, `B` (also `H`), `C`, `D`, `E`, `F` or `G`.

Modifiers: one of `#` or `b`.

Example:
`E#`, `C`, `Gb`

### Duration
**Note**: `[<expr>]` indicates an optional value
```
<unsigned integer> ['/' <unsigned integer>]
```
Examples: `1`, `1/2`, `4 /   2`, `2`

### Ident
Identifiers consist of many characters of `'A'..'Z'`, `'a'..'z'` and `_`.

Numbers are not allowed in identifiers, so `Hello12` should be interpreted as `Hello`, `12`.
# Blocks
### Notes
Syntax:
```
<Note> <Duration>
```
Examples:
```
Ab  1  / 2
B#2/ # same as B#2
G    2/3;A2
```
### Pairs
```
<Ident> <Ident>
```
Examples:
```
goto main
dec var
```

### Variables
```
<Ident> ':' <expr>
```

# Commands

### `goto`
goto a label and after executing it jump back into the previous label.

### `dec`
decrement a variable's value.

### `inc`
increment a variable's value.

# Runtime
A musical compiler should be able to play audio while compiling the code, and also export the played audio as a `wav` file or any desired audio format.

Implementation can vary.