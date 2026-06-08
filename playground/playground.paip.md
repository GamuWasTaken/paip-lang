
# [] : `scope` / `product`
 - A `context` is just a product of `variable`s
  - ? <> : sum type (Can be constructed with [] and any) (maybe suggar)
 - Defines a scope inside of it
 - (@) refers to the current scope (the closest one)
 - (::) accesses the elements of a scope
 - (@::name:type=value) defines a field in the current context
 - return(@>), products return the last value, therefore putting (;) at the end makes the last element implicitly []
 - scopes inherit elements from parent scope, use cow
  - scope has access to every element in parent but modifications stay in the scope, the cannot leak
 - __builtin__
  - run: receives all the names it uses
 - the type of a scope is inferred by the names it uses
  - if we reference a name inside a scope, then, to be able to run it we need said name
 - A scope is a list of expressions/statements

# `variable`s are values created like name:type
 - they live in a `scope`
 - __builtin__ get & set(=)

# `function`s are values
 - its type is built with (>)
 - (>) infix takes 2 `product`s returns type of function from former to later
 - (.) first-ternary-infix operator, value1.function value2
   - joins the 2 values in a temporary product, binds it to the function scope and runs it
   - ? function.run [value1, value2]

# () : precedence


----
products are lists of expressions, separated with (;)
implicitly define a new scope that inherits the parent one, but uses cow -> Changes cannot escape the scope
we can represent them with an:
 - Array + Hashmap
  - If the product does not have named values just use an array :)
  
do we want statements (aka pure side effect no value)
One interesting thing is to have idempotent side effects (not linked to repetition)

products are half indexable, with the var and ctx constructs
does it make sense to index them by number?
how do we do arrays? (builtin to call a product with each element of a scope)

how do we do traits?
what are types?

what about indexing function parameters, i mean, functions are products where not all the values are bound
doing this we 'link' the parameters, meaning if you pass it to one you can pass it to the other

maybe its interesting to have a convention to differenciate functions from simple products?

How do we deal with single-name multiple-type vars?


There is going to be several targets (debug, prod, ...) so we can optimize for each
multiple types for a name

do we handle variadics? (or just pass a product with all the args)
there needs to be a type describint products
 - we want to be able to have arities defined

----
```python

@::user: String = "iker";

#* the `@::name` couldnt be `name`, the @:: forces the caller to explicitly pass a name var, it wont take it from the parent context *#
@::print_name: [@::name: write::data ; std::io::[write;stdout]]>[] = [
 stdout.write [@::name];
];

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
