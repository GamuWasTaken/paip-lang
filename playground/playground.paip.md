# The core has only names and blocks

# Names
 - simply a string of alphanumeric + _ chars
 - it is used to reference elements

# Blocks
 - simply a list of elements
 - defines an internal scope
 - can reference elements from outside, but on mutation they are adopted (mutation does not leak)
 - blocks dont own their elements, they just namespace them

# [] : `block` / `product`
 - A `context` is just a product of `variable`s
  - ? <> : sum type (Can be constructed with [] and any) (maybe suggar)
 - Defines a scope inside of it
 - (@) refers to the current scope (the closest one)
 - (::) accesses the elements of a scope
 - (@::name:type=value) defines a field in the current context, that is accessible from outside
 - return(@>), products return the last value, therefore putting (;) at the end makes the last element implicitly []
 - blocks are lists of expressions separated by (;)
 - scopes inherit elements from parent scope, and use cow
  - scope has access to every element in parent but modifications stay in the scope, they cannot leak
 - __builtin__
  - run: receives all the names it uses
 - the type of a scope is inferred by the names it uses
  - if we reference a name inside a scope, then, to be able to run it we need said name
 - A scope is a list of expressions/statements

# `variable`s are values created like name:type
 - (@::var) makes the var visible from outside
 - (=) assigns a value to the var
 - vars are unique by name+type
 - __builtin__ get & set(=)

# `function`s are values
 - its type is built with (>)
 - (>) infix takes 2 `product`s returns type of function from former to later
 - (.) first-ternary-infix operator, value1.function value2
   - joins the 2 values in a temporary product, binds it to the function scope and runs it
   - ? function.run [value1, value2]

# () : precedence
- Do we even need precedence disambiguation?

# (origin::)
 - on names means `the name 'name' attached to origin`
 - on lists of names applies to each of the names
# there is no opaque state (@ doesnt remember where it was first written)

----
we can index by number, and it returns the expression in that position, without evaluating it
- The an interpreter just iterates the outer block and evals each line :O

how do we do traits?
what are types? empty variables :)

Types are boxes for their possible values, when defining a function (aka accessing @::name inside a block without giving it a value) we are specifiying a parameter of the function, with an explicit or implicit type (depending on how we use it), in this way, the type of a function is of the form references_made>output

Types define a function from Self to unit that panics if the provided element was not of that type (assert)

And therefore a type is just an empty var, we could attach it a function to check if the given value is of the correct type


There is going to be several targets (debug, prod, ...) so we can optimize for each


The possible expressions in the lang are:
- name
- block ([value;*])
- typing (name: type), TODO define type as a value
- assign (name = value), TODO define value as value
- return (@> value), pops the current block with value (implicit at the end of the block?)
- calling (value.value block?), adds the first value to the block, binds it to the second value and calls it

----
```python
# each file is a block (maybe we can import by filename?)

@::user: String = "iker"; # Attaches user:String to the file block

# the `@::name` couldnt be `name`, the @:: forces the caller to explicitly pass a name var, it wont take it from the parent context
# the type says, from a block that has `name` of the type std::io::write::data in a scope with std::io::write and stdout defined to []
@::print_name: [@::name: write::data ; std::io::[write;stdout]]>[] = [
 stdout.write [@::name];
];
# The above function type is not fully defined, std::io::write only has the following limitations:
# - it must have an attached var called data that is a valid type
# - it must accept a parameter of the type data

#* name:type creates a var and adds it to the scope, var=val binds the value to the var *#
@::id: [@::x: Int]>[Int] = [ @> x ];

user.print_name; # ok
[].print_name; # not ok, name is only looked for in the provided context, and its empty

3
 .add [4]
 .mul [5]
 .write [@::sink=std::io::stdout]
;

```
