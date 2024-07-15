
> Outdated to some parts

# Theorie

There's registers built into the language. They prefixed with `r` and enumerated in the format `r<n>` where `n` is a positive integer (`r1`, `r20`, ..). Move instructions (`mov eax,1`) has dedicated syntax using thin arrows (`eax <- 1` or `1 -> eax`). A numerical value can be used as a pointer using `*` as in other low-level higher level languages such as C or Rust.

The primary structure is a list `{}`. Not 'list' as in linked list, but rather as what other programming languages would consider to be a vector. Lists are essential in exas:
The whole program is just a list of top-level instructions. A block is also just a list of instructions, and a function is just a block with parameters.

## Elements
Programming elements or simply elements refer to one of the following:
- Values
- Instructions
- Blocks

## Lists
Syntax: `{<el>, <el>, ..}`

Lists are written as a comma separated listing of elements, enclosed in `{}`.
Blocks are a specific type of list, where the elements are usually consisting of instructions rather than values.

## Variables / Aliases
Every block or value can be named: `<name> = {<inst>, ..}`. This should be thought of primarily as an alias, not a variable in the usual sense (use registers or heap memory).

Being just an alias, whenever the name is used, it's value is essentially copy-pasted. A usual usecase is to assign a name to the address of a block or large value:
```exas
say_hello = &{
    print "something",
},

msg 'message to print' = &{
    "hello",
    "world",
    "!",
},
msg_len = $ - msg, /.. $ is the current address ../

goto say_hello,
```

## Comments and docs
Syntax: `'comment'` | `/.. comment ../` (hidden for docs) | `///`
- Inline comments / labels: `'` (`comment', 'this is a comment'`)
    - Have to be delimited on the same line
    - Can be placed nearly anywhere
    - Appear in the docs
- Block comments: `/.. ../` (`/.. multi line comment ../`)
    - Have to be delimited
        - Not necessarily on the same line
    - Can be nested
    - Not in the docs

- Docs: `///` (`/// documentation comment ///`)
    - **Have to be delimited** by a matching `///`
        - Not necessarily on the same line
    - Can be nested
    - Appear in the docs of the *next* element

## Functions

Functions are just blocks which take parameters. In order to understand functions, we need to understand blocks first.

### Blocks
Syntax: `{<inst>}` where `<inst>` is a list of instructions.
```exas
'main' {
    0 -> eax,
    eax -> ebx,
}
```
The entire block will be evaluated to the last value, unless it one is delimited by a comma `,`.
```exas
r1 <- { if x < 0 { x + 1 } else { 4, } }
```

In the case of `else` here, the unit value will end up in `r1`, not `4`.

### Function definitions
Syntax: `(<args>){<inst>}`
```exas
add = (x, y) {
    x + y
}
```

### Function calls
Syntax: `<name> <args>`

Every token after the name is considered an argument and passed on to the function. An exception are parentheses, who are evaluated first and then passed on as a single argument. For instance, in `f 2 + 4`, 3 arguments are passed, `2`, `+`, and `4`. If the expression should be evaluated first, parentheses can be used `f (2 + 4)`.
Blocks can be used as arguments too.

> To call a function, the parentheses and commas are **not** required. In fact, `f a b` is not the same as `f (a, b)`,

## Types
idk yet.

I'd like:
- types implemented in the stdlib rather than by the compiler
    - comptime functions doing type checks?
- a way to express linear types / higher RAII
- nice errors as values
- options
- or well notated nullables?
    - -> more performant

## Extras

How the `if` function could be easily redefined using just a function:
```exas
if = (x, '>', y, block) {
    cmp x, y
    rt? { block }
},
if = (x, '<', y, block) {
    cmp x, y
    lt? { block }
},
..
```

Or if-else:
```exas
if = (expr, if_block, 'else', else_block) {
    if expr { if_block },
    if !expr { else_block }
},
```
For performance reasons, `if` it isn't actually defined like that.






