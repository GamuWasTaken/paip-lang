
# [] : `block` / `product`
 - A `context` is just a product of `variable`s
  - ? <> : sum type (Can be constructed with [] and any) (maybe suggar)
 - Defines a scope inside of it
 - (@) refers to the current scope (the closest one)
 - (::) accesses the elements of a scope
 - (@::name:type=value) defines a field in the current context, that is accessible from outside
 - return(@>), products return the last value, therefore putting (;) at the end makes the last element implicitly []
 - bloks are lists of expressions separated by (;)
 - scopes inherit elements from parent scope, use cow
  - scope has access to every element in parent but modifications stay in the scope, the cannot leak
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


----
do we want statements (aka pure side effect no value) (in a way not attaching a var to the context is a statement, as its value can never be accessed again)
We need sugar for arrays having to put @:: in front of each element is crazy
- Well we could allways do, @::[a,b,c...], like we have done for write and stdout below
 - We can do that because ::name means attach ::[...] means get all of them (we can treat it differently depending on if we do it on a name or on @)

products are half indexable, with the var and ctx constructs
we can index by number, and it returns the expression in that position, without evaluating it
- The an interpreter just iterates the outer block and evals each line :O

how do we do traits?
what are types? empty variables :)

Types are boxes for their possible values, when defining a function (aka accessing @::name inside a block without giving it a value) we are specifiying a parameter of the function, with an explicit or implicit type (depending on how we use it), in this way, the type of a function is of the form references_made>output

And therefore a type is just an empty var, we could attach it a function to check if the given value is of the correct type


There is going to be several targets (debug, prod, ...) so we can optimize for each


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
