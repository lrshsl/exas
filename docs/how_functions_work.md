
# Functions | macros - pattern matching

Functions have a **name**, **parameters** and a **body**. The following is the
definition of at function called `fn-name` that takes two arguments, `arg1` and
`arg2`.

```exas
fn-name = (arg1 arg2) { }
```

Arguments are matched literally unless `[]` are used. Thus functions in exas
can do more than functions in most languages (think functions and macros
combined).

## Match literals and symbols

This means that the following function call:
```exas
fn-name "str lit" ident 5
```

matches the following function declaration:

```exas
fn-name = ("str lit" ident 5) { }
```

There are more matching function declarations, but this is the most specific
possible. Thus, no argument is actually an argument, but rather simply matched
against. The above declaration would not match if the string literal, the
identifier `ident` or the number were any different. None of these would match:

```exas
fn-name "another" ident 5,
fn-name "str lit" identt 5,
fn-name "str lit" ident 6,
```

## Match types

Usually, functions are declared less explicit, e. g. through their types:

```exas
fn-name = ([:Str] [:Ident] [:Number]) { }
```

All of the above function calls would match this function declaration.


## Parameters

Even more useful is it, to being able to access the values of the arguments in
the function body. For that, they can be given a name:

```exas
fn-name = ([arg1: Str] [arg2: Ident] [arg3: Number]) {
    print arg1 arg2 arg3,
}
```

## Where clause

You know Rust? Rust is cool.

```exas
fn-name = ([arg1] [arg2] [arg3])
    where
        arg1: Str,
        arg2: Ident,
        arg3: Number,
{
    print arg1 arg2 arg3
}
```

## Types are also just comptime functions

A type is a function that checks its value. In the example above for instance,
`Str`, `Ident` and `Number` are functions that are applied on `arg1`, `arg2`
and `arg3` respectively. This is done at compiletime, and the functions will
throw a miss matched types error when compiling.

This means that more than just the usual type information can be encoded into
an exas type. Let's create some nice numeric types for example:

```exas
type Unsigned = ([a: Any]) {
    is (typeof a) .. {

        .. u8 | u16 | u32 | u64 | u128 | usize ?
            true,

        .. other? {
            comptime-error typeMismatch other " is not an unsigned number",
            false
        },
    }
}

type Signed = ([a: Any]) {
    match (typeof a) {
        .. i8 | i16 | i32 | i64 | i128 | isize => true,
        .. other => {
            comptime-error typeMismatch other " is not a signed number",
            false
        },
    }
}

type Number = ([a: Any]) {
    is (typeof a) {
        .. Unsigned | Signed | Complex?
            true,
        .. other? {
            comptime-error typeMismatch: other " is not a number",
            false
        },
    }
}
```

> Note: The syntax of `match` | `is` statements is not consistent. It is
> unclear yet, which version it's gonna be.

The `type` keyword guarantees that the function runs at compile time, returns
`bool` and may throw a `comptime-error`.

# Why?

This is the cool part about it:

```exas
add = ([n: Number] to [var: Ident]) {
    var += n,
}

x = 4,
assert x == 4,

add 4 to x,
assert x == 8
```

Or, recreating the `let` (keyword | function | macro):

```exas
let = ([name: Ident] = [value: Any]) {
    /.. Some magic ../
}
let x = 4, /.. Syntax recreated! ../
```
