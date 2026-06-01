
[] : product type
 - A `context` is just a product of `variable`s
 - <> : sum type ? (Can be constructed with [] and any)

{} : `scope`
 - global scope + 1 for each {}
 - (@) refers to the current scope (the closest one)
 - (::) accesses the elements of a scope
 - scopes inherit elements from parent scope, use cow
  - scope has access to every element in parent but modifications stay in the scope, the cannot leak
 - __builtin__
  - run: receives all the names it uses
 - the type of a scope is inferred by the names it uses
  - if we reference a name inside a scope, then, to be able to run it we need said name

`variable`s are values created like name:type
 - they live in a `scope`
 - __builtin__ get & set(=)

`function`s are values
 - its type is built with (>)
 - (>) infix takes 2 `product`s returns type of function from former to later
 - `function`s as traits?
 - (.) first-ternary-infix operator, value.function value
  - joins the 2 values in a temporary product, binds it to the function scope and runs it

() : precedence


----

Do we need/want something to mark names so they are not looked in a context,
- a simple name is looked in every context bottom up (current, parent, parent parent...)
- we can cuallify a name with scope::name to only search in that scope
 - therefore @::name refers to 'name' in the current scope

- How do we return from a scope?
 - @> value

- Append functions to types (types are products?)

----

user: String = "pepe"
print_name: [name: std::io::Print]>[] = {
 @::name.print
}

id: [x: Int]>[Int] = { @> x }


user.print_name
