
# Paip base

# `{}` body (exponent)
defines a new *context* of *variable*s

## Argument resolution
If type of body is provided, order and type are used
Else only type
If not enought for resolution an error is raised asking for clarification
Once an argument has been resolved its parameter is marked as full and it is not counted in further resolutions
	This means we only need to bind/name the ambiguous parameters and the rest can be passed cleanly
	MAYBE mark arguments with $ or something like that (looks php yuk)

## Scoping
A name not found in the current context is searched for in the parent

# `()` context (product)
serves for structs and arrays, is a simple list, if it contains just *atoms* then we can treat it as a struct
we then can optimize for structs with a different backing ds
() is unit
elements can be added and removed from a context

# `<>` enum (sum)

# ` ` implicit call
a function followed by another value is treated as a call

# `.`  call (context application)
moves the value in front to the end of the function call

# `'`  name (name association)
binding a name to a value generates an *atom*
bindings can only have depth 1, if you bind a name to a name, you are just binding the value of the second name to the first

# `:`  type (type association)

# Tokens
`{}()<>.':` -> language elements
`[a-zA-Z_]+` -> names

__built-in__
- `let` adds atoms to the current context
- `atom` a name and a value
__std__
- `variable` (name'string, value'top, type'?) // TODO what type is type?

greet'{
	name.print
} : (name'string) -> ()


"juan".greet
