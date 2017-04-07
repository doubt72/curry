# Curry

Another toy language interpreter.  The 2.5th-most pointless thing I've ever
done.

This language is more or less based on
[doubtful](https://github.com/doubt72/doubtful), another fairly pointless toy
language.  Both were written as an experimental (learning) project after I got
inspired by reading compiler papers because I didn't know much about compilers
and was curious.  Up to that point I'd never done anything about compilers at
all, I'd been on the operating system track instead as an undergraduate
(although I didn't finish a computer science major, it wasn't because I didn't
have enough computer science credits, I got a different degree instead).  In
hindsight, reading those papers and things may have been a poor decision, but I
guess it passed the time between interviews.

Anyway, I did learn some interesting (to me) things about language design in the
process, not that the result is likely useful for anyone else.  (For the record,
doubtful was just a super simple functional language.  After making some changes
to this one in an attempt to make things even simpler and more streamlined, I
ended up with something a lot more like Scheme with terrible syntax. Um...
Yeah). So far it hasn't actually produced an actual compiler, just this
interpreter, though I've been idly thinking about hooking it into LLVM for shits
and giggles and excruciating pain.

I also (mostly) learned Rust in the process.  Because why tackle any significant
project without learning a new language you've never even looked at before at
the same time?  Of course, that also means that this is perhaps not the best
Rust program ever produced... It was my first Rust project, after all (it's the
same code I as I used for doubtful, just further evolved), I learned how to do
things (not *always* the best way) as I went, and didn't go back to clean
everything up.  I also learned why Rust is such a great language for what it is
and how despite that you should probably never actually use it in real life.
Unfortunately, that means there still isn't a perfect language out there for
complex yet low-level abstraction (Rust is so close that mostly it's just
frustrating), and now I'm really, _really_ sure I'm not going to write it.

Why Curry?  Because Curry is delicious (not saying which is best, even Japanese
curry has Coco Ichi, but we all know that it's Indian.  I give you Raj Mahal
in 東京 as proof).  Also, there is no currying in this langauge.  It's not even
possible, technically speaking.  Clear?  Clear.

Also, it has code so "beautiful" that will make you `.cry`

Sorry.  (Not sorry.)

## Example

Here is an example of Curry code:

```
# Map function implementation:
map:
  list:
    car[__];
  ;
  func:
    car[cdr[__]];
  ;
  ?[=[cdr[list] []] ~[[,[func[] [car[list]]]]] []];
  +[[,[func[] [car[list]]]] map[cdr[list] func]];
;

# Usage Example:
add_one::+[car[_] 1];;;
map[[1 2 3] add_one]; # Returns [2 3 4]
```

Pretty obvious, huh?  Some of that white space isn't even necessary, but I'm here for
you, reader person.

## Design

### Types

Curry is strongly typed, but type is implicit.

#### Scalar types:

* **Atom**: `true`, `false`
* **Integer**: 64-bit integer
* **Float**: 64-bit IEEE blah blah.  Don't worry about it, it's got a dot in it.
  Sorry European readers.
* **String**: UTF-8 string; length primitive returns number of codepoints, not
  bytes.  Double-quotes are used for literals.

Literals examples: atom: `true`, int: `0`, float: `0.0`,
string: `"0"` (so far, so simple).

#### Other types:

* **List**: lists are collections of expressions of arbitrary types.  Internally
  they're vectors, but they're manipulated as if they were S-expressions
  (actually implementing S-expressions in Rust turned out to be way more trouble
  than it was worth). I kinda got Lisp all over my language. Can't seem to get
  the stains out.  It got everywhere except the places it's not.  Can contain
  any types in any order.
* **Function**: functions are a first-class type. Then again, there aren't any
  other kinds of types, so they're just a type, I guess.  Functions take a
  single argument (which must be a list).
* **Exception**: used for flow control, since the language is an iterative
  functional language (um), the only flow is the sequence of expressions in a
  block, and the only flow control is the exception, which terminates the block
  (or the program) if uncaught. Return is a special type of exception that that
  is swallowed by the block before returning the exception payload.

As an aside, I don't know that anyone has ever used exceptions as the sole way
to do control flow in a language before, but then, I don't actually know all
that much about language design, so I wouldn't, would I.  My lack of knowledge
may also explain the deep inner "beauty" of this language.

Anyway, here's a literal example of list: `[1 2 3]`.  There aren't any literals
of exceptions, and an anonymous identity function could look like this (it's not
*exactly* an identity function, though, because well, reasons.  Identity
functions aren't exactly...  Well, you can only pass lists, so...  But this
*looks* like an identity function, oh, never mind, this is an anonymous
function):

```
:car[_];;
```

### Syntax Things:

The basic unit of code is a block, which is a sequence of expressions.
Expressions in a block must be terminated with a semicolon (`;`); inside a list
they're terminated by whitespace or the end of the list.

Calls are any id (i.e., something that wouldn't be parsed as a literal atom,
number, string, etc.), optionally followed by a list.  If no list is specified,
an empty list (`[]`) is passed.

Definitions are just a normal expression, and can be found anywhere including
inside other function definitions (in which case they are scoped to the
function).  They may or may not have an id (and if not, they're anonymous, in
which case they're gone forever if not returned by a function).  They are
followed by a colon (`:`) and a function block, and also terminated by a
semicolon (`;`) like any other expression (which is why functions in a block
are always followed by at least two semicolons, one for the final expression in
the function block in the definition, one for the definition itself.  This
includes the main program, which is just another block).

There are no variables, only defined functions and `_` which is the list
(parameter) passed to the function.  The parameter of the parent function is
stored in `__`, and so forth and so on to the main block (where it's an empty
list).

Definitions are immutable!  Except inside a block they'll hide any definitions
from the enclosing scope(s) calling the block/function.  Blocks aren't really
closures in any sense, the context/scope of a function isn't stored when a function
is defined, it's dynamically generated at runtime.

#### Differences from doubtful

The main difference is that I got rid of parameters.  All functions now take a
single parameter (a list).  If nothing is passed, calls implicitly pass an empty
list.  I also got rid of nil; an empty list (`[]`) is now used for the same
thing (it only really matters for list operations anyway).  That means no more
parenthesis.  It also means that lists more or less behave like S-expressions
now as well (or at least the system functions manipulate them that way a bit
more consistently), although internally they're still just vectors, because
optional recursive data functions are a pain in Rust.

Yes, this makes the language a lot more Scheme-like, I'm glad you noticed.  It
also _radically_ simplifies parsing and evaluation.

## Reserved Characters:

The following characters have special meaning: `:` `;` `[` `]` `"` `#`

Anything else can be used in a function name.

`#` is used for comments, to the end of a line.

## BNF:

Have some BNF:

```
<block> ::= [ <expression> ';' ]*
<expression> ::= <definition> | <call> | <literal>
<definition> ::= [ <id> ] ':' <block>
<call> ::= <id> [ <list> ]
<literal> ::= <scalar> | <list>
<list> ::= '[' [ <expression> ] [ <whitespace> <expression> ]* ']'
<scalar> ::= <atom> | <int> | <float> | <string>
<atom> ::= 'true' | 'false'
```

For brevity's sake, not defining ids int, float, string, or whitespace here.
Strings are double-quote delimited (currently there are no escapes), whitespace
is whitespace, and numeric types are whatever Rust can successfully parse as
such, ids are everything else (even weird shit like `0z_f` or whatever).

## Primitives:

### Operations:

* All numeric types: `+`, `-`, `/`, `*`
* Integer only: `%`
* Boolean: `&`, `|`, `!`

### Comparisons:

* Numeric types only: `<`, `>`
* `=`: any dissimilar types are not considered equal, comparisons of functions
  is always false

### String Operations:

`substr`, `strlen`, `+` (concatenation)

### List Operations:

`car`, `cdr`, `+` (cons)

### I/O:

* `>>`: outputs a string
* `<<`: _not implemented_

### Type Conversion:

* `int`: float or string to int
* `float`: int or string to float
* `string`: pretty much anything to string (except exceptions)
* `list`: _not implemented_

### Others:

* `,`: executes an anonymous function (`car[_]` must be function, `cdr[_]` is
  passed to that function)
* `?`: if `car[_]` is true, returns `car[cdr[_]]`, else `car[car[cdr[_]]]`
* `raise`: raises an `error` exception with `car[_]` as payload
* `catch`: catches an exception and returns a list that looks like
  `[<type> <payload> <stack>]`, if passed non-exception expression, returns
  `["ok" car[_]]`
* `~`: returns `return` exception (which is swallowed by block which returns
  `car[_]` of `~`, i.e., the `return` payload)

## Possible Primitives (not implemented):

Besides `list` and `<<`: math primitives (`sqrt`, trigonometric functions, and
the like).

## Not Primitives:

The following can be derived from other primitives: `>=`, `<=`, `!=`, `^`
(exclusive or), `pow`, `truncate` (lists), `$` (index), `sub` (lists), `len`
(lists), any variations on `cadr` or `caddr` etc., `@` (map), `.` (from, to),
`pi` (any constants).

They'd be faster as primitives, but Curry is *pure*.  Pure evil,
because curries are supposed to be *spicy*.  Delicious!

## See More

There is [sample source](test.cry) with a whole bunch of tests.  Depending
on the state of the rest of the code, sometimes it works.

Install Rust and run this to see:

`cargo run test.cry`

## TODO:

Maybe:

* Hashes
* Better error handling for parser, keep track of line numbers, etc
* Optimize tail recursion
* Math primitive
* String escape codes
* Refactor and clean all the shit up?
* Build LLVM compiler
