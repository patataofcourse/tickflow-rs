# Tickscript specification
### version 0.1.0

by patataofcourse

> Note: a "language implementation" refers to a program that reads Tickscript code and converts it to a specific bytecode, with its own set of commands and limitations.

> While this is the finished version of Tickscript specification v0.1.0, it is not meant for use yet. Many breaking changes will likely follow over the next few months.

## Values 

Tickscript defines the following types of values:

### Integers

Tickscript supports integers in the following radixes:

- Decimal: `0`, `1`, `549023`
- Hexadecimal: `0x29A`
- Octal: `0o31536`
- Binary: `0b10110`

#### Typing

The default integer type is `int`, which is equivalent to `u32` (32-bit unsigned) in most cases, however, in an arg0 position, it acts as an 18-bit unsigned integer.

Other integer types are available:

| Sign     | 8-bit | 16-bit | 32-bit |
| -------- | ----- | ------ | ------ |
| Unsigned | `u8`  | `u16`  | `u32`  |
| Signed   | `i8`  | `i16`  | `i32`  |

`s8`, `s16`, and `s32` are not integer type names.

### Strings

Strings can be values in Tickscript, with an optional prefix `u` for UTF-16 strings: `"hello ascii"`, `u"hello utf-16"`.

Other optional prefixes for strings can be added in the future (such as `b""`, `r""`, etc).

- To display a `"` character in a string, you must escape it with a backslash: `\"`
- To display a backslash, you must escape it as well: `\\`
- Newlines are represented with `\n`

#### Typing

Strings are covered under the `string` type.

### Arrays

Tickscript supports arrays, which can contain any number of elements of one type. They are immutable (like all other types in Tickscript), and can contain strings or other arrays, as long as those strings or arrays are all of the same type.
> This means, for example: you can't store a UTF-16 string and a ASCII string in the same array, or a u16 array and a u32 array in the same array.

For integer arrays, you can specify the type of the integer (see [above](#typing)). Otherwise, it'll be assumed to be `u32`.

Examples: `[0, 1, 2]`, `u16[5, 9, 65535]`, `["hello", "world"]`, `i8[-127, 0, 127]`.

Trailing commas (e.g. `[0, 1, 2,]`) are allowed in arrays.

Here's an example of nested arrays:
```c
[
    // Try Again
    [
        null, // JP
        "hello", // EN
        ...
    ],
    // OK
    [
        null, 
        "goodbye",
        ...
    ],
]
```

#### Typing

Arrays are represented by a type `x[]`, where `x` is the type of the contents of the array.

### Sub pointers

Sub pointers can't be created manually. Instead, they are the type of references to a specific sub by name. See [subs section](#subs) for more information.

#### Typing

Sub pointers are represented by the `sub` and `sub_sync` types, for asynchronous and synchronous subs, respectively.

## Identifiers

Identifiers in Tickscript are restricted to alphanumeric characters and underscores (`_`). The first character in an identifier cannot be numeric.

Identifiers, when used as arguments of a command, must refer to an existing constant or sub
 
    - In the case of it being a constant, it will resolve to the value of said constant
    - In the case of it being a sub, it will resolve to a pointer to said sub
  
No two items (constants, subs, etc.) can share the same identifier. If they do, the compiler will emit an error.

### Namespaces

Tickscript defines a namespace as an identifier that represents a group of values. You can access a value inside of a namespace like so: `namespace.element`.
> This can be nested as much as is required.

### Keywords
Keywords are reserved identifiers that cannot be used as a sub or constant name, due to already having a meaning in the language.

- `true` (integer constant equals to 1)
- `false` (integer constant equals to 0)
- `null` (integer, array, or string constant equals to 0)
    - This is not to be used lightly, as it can easily cause a crash
- `sub`
- `sync`
- `const`
- `if`
- `else`
- `switch`
- `case`
- `default`
- `break`
- `raw_op`
- Every type name: `any` plus all the types defined in the [values section](#values)
- Command names are also treated as keywords for the sake of clarity, however, those names vary between language implementations

## Operations

Tickscript supports compile-time operations to be done to constant values. The available operations are the following:

### For integers

The following binary (two-operand) operations are available:

- Addition: `a + b`
- Substraction: `a - b`
- Multiplication: `a * b`
- Integer division: `a / b`
- Shift left: `a << b`
- Shift right: `a >> b`
- Bitwise AND: `a & b`
- Bitwise OR: `a | b`
- Bitwise XOR: `a ^ b`

In addition, there's also two unary (single-operand) operations:

- Negation: `-a`
- Bitwise NOT: `~a`

### For strings

Strings only have one operation available to them: concatenation. It uses the addition symbol: `a + b`.

## Statements

Statements are the main component of Tickscript. A statement is a basic building block of a Tickscript file that defines the different values and commands in a chart.

Most statements are terminated by a line break. It is possible to add several statements in one line by using a semicolon as a terminator instead. However, this is only recommended for particularly short constant definitions or command statements.

A line doesn't necessarily have to contain a statement, it can be empty or consist of only comments.

The first statement in a Tickscript file (that is, the first non-empty line excluding comments) must be a `#tickscript` directive (see [directives section](#available-directives)).

There's four main kinds of statements: directives, subs, command definitions, and constant definitions.

### Directives

Directives are easily identifiable due to starting with a `#` symbol. They are similar to C preprocessor directives, however, they define values that are intrinsic to the Tickscript file, such as metadata.

Directives use the following syntax: `#name arg0, arg1, ..., argn`. The name is a unique identifier for the directive one wants to use. The number of arguments and their accepted types is determined by the directive in question.

Directives are the only kind of statement that requires a line of its own. It can't be terminated with a semicolon or be placed after another statement with a semicolon.

#### Available directives

- `#tickscript`: this directive is required for the file to be valid Tickscript. It must be the first statement in any Tickscript file, and is invalid in any other location.
- `#include <filename>`: makes the contents of the file specified by `filename` available to this file. Requires the included file to have an `#includeme` or `#module` directive. Filename must be a string.
- `#includeme`: specifies this file can be included and cannot be compiled on its own.
- `#module <namespace>`: similar to `#includeme`, however, instead of being directly accessible, the elements of the file will be available in this file under a namespace `namespace`. Namespace must be an identifier.
- `#requires <version>`: defines which version of the Tickscript spec this file follows. Version must be a string of the format `"x.y"` or `"x.y.z"`, where x-y-z are integers. Compatibility between a file and a spec version is as follows for any version x.y.z:
  - `x` must always match between the two versions
  - if `x == 0`, `y` must always match between the two versions and the spec's `z` must be greater or equal than the file's.
  - if `x != 0`, the spec's `y` must be greater or equal than the file's, and, if both `y` values match, the spec's `z` must be greater or equal than the file's.
- `#tempo <id> <samplerate>` + `#endtempo`: mark the start and end of a tempo section. `samplerate` is optional, and defaults to 32000. Tempo format is a set of lines formatted like so:
    - TODO: define tempo format
- `#index <index>`: sets the default index in a generated mod manifest. Only for non-includable Tickscript files.
- `#name <name>`: name of the mod or file. Will be included in generated mod manifests.
- `#authors <authors>`: authors of the mod or file (`string[]`). Will be included in generated mod manifests.
- `#description <description>`: description of the mod or file. Will be included in generated mod manifests.
- `#version <version>`: version of the mod or file. Will be included in a generated mod manifest. Semantic versioning preferred but not required.

### Subs
Short for "subroutines", subs define the Tickflow bytecode to be compiled. Similar to functions in most programming languages, they define a set of instructions to be ran in order.

Subs are declared with the keyword `sub`, for example: 
```c
sub my_sub {
    // sub contents here
}
```

Subs are asynchronous by default, that is, they spawn a new thread. You can define them as synchronous by adding the keyword `sync` before `sub`, like this:
```c
sync sub my_sync_sub {
    // sub contents here
}
```

A sub's contents is a list of statements, however, only two unique kinds of statements are allowed within subs: command statements and syntactic statements. These **cannot** be used outside a sub's contents!

#### Command statements
Commands are the direct representation of Tickflow, abstracted for easier understanding. They follow this syntax:

```c
command_name arg1, arg2, ...
```

If you have any prior experience with Tickflow, it might surprise you to not see the arg0 (also known as special argument) included in this syntax. This is because, aside from [raw-op commands](#raw_op-commands), arg0s are not manually managed: they should be handled as a regular argument or as different commands, depending on the function of the arg0.

The language implementation has the choice and ability to make its commands as abstracted and simplified as it wants, for the sake of easiness, simplicity, and ergonomics. Optionally, it can supply raw variants of the commands it abstracts.

If a specific command isn't defined in the language implementation, you can use a [command definition statement](#command-definitions) to define it.

##### `raw_op` commands

Raw Tickflow commands (with the Tickompiler syntax `cmd<arg0> arg1, arg2...`) can also be used with the `raw_op` keyword, like so: `raw_op cmd<arg0> arg1, arg2...`

You can use this to run commands that aren't defined in the language implementation.

However, you can't use *named* commands in `raw_op` command statements. `cmd` **must** be an integer.

#### Syntactic statements
Syntactic statements are the abstraction of some concepts commonly used in Tickflow, such as if/else statements or loops.

If they do not exist or cannot be represented in a specific language, the implementation for it can manually disable them.

##### Available syntactic statements
> Note: since tickflow usually works with a conditional variable, condition-based statements work by applying a specific comparison to that specific variable and a given constant value (condition)
> 
> If this ever changes (for example, in a "Rhythm Heaven 5", as long as it still uses Tickflow), the spec may be updated.

> Note: strings and arrays **cannot** be compared to each other unless the language **specifically** features string/array comparison.


- `if` / `else if` / `else` statements:

```c
if <op> <condition> {
    ...
} else if <op> <condition> {
    ...
} else {
    ...
}
```

Available operations are: `==` (equal), `!=` (not equal), `>` (greater than), `<` (less than), `>=` (greater or equal to), `<=` (lesser or equal to). Specifying no operation defaults to `==`.

- `switch` / `case` statements

```c
switch {
    case <condition>:
        ...
        break
    case <condition>:
        ...
    case <condition>:
    case <condition>:
        ...
        break
    default:
        ...
}
```
              
The final `case`/`default` in the statement does not need to include a `break` statement.

- Inconditional loops (`do N` loops):

```c
do <N> {
    ...
}
```

This will expand the code inside the loop the specific amount of times required (if the language does not already include an inconditional loop function).

`do N` **must not** alter any runtime variables, or if it does, it must do so in a way that the Tickscript user must not be able to interact with. 

- Conditional and infinite loops (`while` and `loop`)

```c
while <op> <condition> {
    ...
}
```

This will repeat the code inside the loop until the conditional variable succeeds in the comparison. Operations are the same ones available in `if` statements.

```rs
loop {
    ...
}
```

This will make the loop go on forever until it is killed (kill_loc, kill_cat, engine switching, etc.)


### Constant definitions

A constant definition gives a name to a specific value, so that it can be used anywhere else in the Tickscript file. Constant definitions follow this syntax:

```c
const <name> = <value>;
```

The name must be any valid identifier (namespaced or not) that is not currently in use, and the value can be any valid Tickscript value. This means it's possible to set a constant to another constant or a modification of another constant, for example:

```c
const one_beat = 0x30;
const two_beats = one_beat * 2;
```

A constant can also be of the type of a sub pointer or an array of sub pointers, in which case each of the values acts like a regular sub pointer.


```c
sub a {

}
sub b {

}
const one_sub = a;
const both_subs = [a, b]

sub c {
    call one_sub;
}
```

### Command definitions

A command definition creates an alias to any existing Tickscript command (including raw Tickflow operations). This is their syntax:
```c
command <new_signature> = <old_signature>
```

The old signature just looks like a regular command or raw_op statement, while the new signature looks like this:
```c
<name> <arg1_name>: <arg1_type>, <arg2_name>: <arg2_type>, ...
```

This applies for any number of arguments between 0 and 15.

The command name has to be a valid identifier (namespaced or not) that is not currently in use. The argument names also need to be valid identifiers not in use, however, they'll only be valid for the context of the old signature definition. The argument types can be `any` or any of the types defined in the [values](#values) section.

You can use any of the arguments defined in the new signature as values in the context of the old signature.

Example:
```c
command rest.beats beats: int = rest beats * 0x30
command call.default _sub: sub = raw_op 0<4>, _sub, 0, 0
```

### Comments

A comment is a sector of the Tickscript file that does not get read by the compiler. This works to leave clarifying notes or documentation.

Line comments follow this syntax:
```c
some_command 0, 1, 2
// this comment covers the entire line
another_command 3, 4, 5 // this comment covers the right-hand side of the double-slash
```

Multiline comments follow this syntax:
```c
some_command 0, 1, 2
/* the comment starts whenever these symbols are found

a multiline comment can span
as much space as needed
as long as it ends with this */
```

For readability reasons, multiline comments **cannot** have any statement directly after in the same line.

```c
/* don't do this, it'll error! */ some_command

some_command // do this instead, it's prettier!
```

## Per-implementation details

Each language implementation would include the following, defined within the compiler code or external Tickscript files:

### Definitions for commands:

This is the most important part, because it's what makes Tickscript worth it - less need to manually work with hex codes, opening up a more accessible language for newcomers and oldies alike.

Command names may be namespaced.

### A standard library

This library would be available for any Tickscript file for this specific language implementation to use as a module.

The module does not need to be called `std`, in fact, it most likely should not, for accessibility to people not experienced with programming. Descriptive names for the specific implementation are preferred. (e.g. `megamix` or `rh3ds` for RH Megamix)

The standard library **must not** be split across several different modules, instead being one unified module, which can then be namespaced.

The contents of the standard library will consist of different subs and constants, made to enhance the tickflowing experience, as well as give some values a meaning through a name. Some example of constants that could be defined are: sub names, button codes, scene or game IDs, etc.

> Note: How standard library subs will be written will be described in future versions of the specification, currently Tickscript as defined by this spec **does not officially support standard library subs**, only constants.

### A runtime
A runtime is a setup system that allows the code to be simplified, similar to Tickflow templates such as EHG's and TheAltDoc's.

A runtime consists of pre-written Tickscript that can be used as a base for less complex charts and projects. It must include references or calls to at least one sub to be defined in the final tickflower's code, and may also optionally include references or calls to more user-defined subs or constants.

For more advanced users, the runtime can be overriden at any time by creating a sub named `_start`, which will override the runtime's original `_start` sub. Ways to make runtime overrides more modular may be included in future versions of the specification.

> Note: how the runtime will be written and distributed will be described in future versions of the specification, currently Tickscript as defined in this spec **does not officially support runtimes**

## What's left?
- Specify a method for runtime and standard library sub creation and distribution
- Should `#includeme` and `#module` be one directive? if so, what could be the name?
    - `#module` is one possible name, however, that could be confusing, since "module" usually involves being wrapped in a namespace in most programming contexts
- Is "syntactic statements" a good name?
- Should `#requires` be required? (Leaning towards "yes")
- Describe the `#tempo` - `#endtempo` format (will be mostly the same as Tickompiler)
- Is the language as defined in this spec too jarring of a change? Does it still include some inaccessibility issues for people unexperienced with code?
- Actually implement the spec!
