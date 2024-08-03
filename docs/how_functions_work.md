# Functions | macros - pattern matching


## Function call

Function calls consist of the **name**. The name is followed by **arguments**.
This is similar to the famous S-Expression syntax from Lisps and similar to
Lisps, everything following the function name is treated as an argument
(everything until the next comma to be precise). What's more interesting about
it is that the arguments are usually not expanded, but rather the *tokens are
passed directly to the function*. There, they can be matched literally or
loosely.

Consider the following example:

```exas
let name = "world",
println (cat "Hello " (name)),
```
Here, `let` and `print` and `cat` are all function calls. The identifier
`name`,  the string `world` and even the equal sign (`=`) are all arguments of
the `let` function.

The parenthesis `()` have a special meaning in argument lists. They tell the
compiler that they have to be *expanded first* and only *then* passed to the
function. This is necessary in the above case, since we don't want those
arguments to be passed as tokens but rather the expanded values.

> Btw, I think the default behavior of `print` should be to `cat` the
> arguments, with a space as a separator. Thus one could simplify the above
> expression `println (cat "Hello " (name))` to `println "Hello" (name)`.

## Function definition

For the definition of a function, the `fn` keyword is used, followed by a list
of parameters. Parameters are a kind of pattern, that also defaults to matching
specific, unexpanded tokens, but can be made more general by using powerful
`[]` parameter expressions.

Some examples:

```exas
echo = fn (first [first_0: Str]) {
    println "hi" first,
},
echo = fn (second [second_0: Str] [second_1: Str]) {
    println second_0 second_1,
},

echo first "hello",             || -> "hi hello\n"               
echo second "hello" "world",    || -> "hello world\n"            

echo fsrit "hello",             || Error: no matching function
echo first "hello" "world",     || Error: no matching function
```

## Match literals and symbols

This means that the following function call:
```exas
fn-name "str lit" ident 5
```

matches the following function declaration:

```exas
fn-name = fn ("str lit" ident 5) { }
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
fn-name = fn ([:Str] [:Ident] [:Number]) { }
```

All of the above function calls would match this function declaration.


## Parameters

Even more useful is it, to being able to access the values of the arguments in
the function body. For that, they can be given a name:

```exas
fn-name = fn ([arg1: Str] [arg2: Ident] [arg3: Number]) {
    print arg1 arg2 arg3,
}
```

## Where clause

You know Rust? Rust is cool.

```exas
fn-name = fn ([arg1] [arg2] [arg3])
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
an Exas type. Let's create some nice numeric types for example:

```exas
Unsigned = type ([a: Any]) {
    is (typeof a) .. {

        .. u8 | u16 | u32 | u64 | u128 | usize ?
            true,

        .. other? {
            comptime-error typeMismatch other " is not an unsigned number",
            false
        },
    }
}

Signed = type ([a: Any]) {
    match (typeof a) {
        .. i8 | i16 | i32 | i64 | i128 | isize => true,
        .. other => {
            comptime-error typeMismatch other " is not a signed number",
            false
        },
    }
}

Number = type ([a: Any]) {
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

The `type` keyword guarantees that the function can run at compile time,
returns a `bool` and may throw a `comptime-error`.

# Why?

This is the cool part about it:

```exas
add = fn ([n: Number] to [var: Ident]) {
    var += n,
}

x = 4,
assert x == 4,

add 4 to x,
assert x == 8
```

Or, recreating the `let` (keyword | function | macro):

```exas
let = fn ([name: Ident] = [value: Any]) {
    /.. Some magic ../
}
let x = 4, /.. Syntax recreated! ../
```
