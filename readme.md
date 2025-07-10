# Bracket - an esoteric programming language 

## Numbers 
- Represented as nested brackets
- 0 -> [] (equivalent to group of no statements)
- 1 -> [()]
- 2 -> [()()]
- 3 -> [()()()]
- ...

## Operators
- `&` (equivalent to `+1`)
- `*` (equivalent to `-1`)

## Variables/Functions
- `{x}()[y]` - set value of variable `x` to `y`
- `{x}[]` - get value of variable `x`
- `{x}(y)[z]` - define function `x` with arguments `y` to return value `z`
- `{x}[y]` - call function `x` with arguments `y`
- statements can be grouped using `[]` (value of a group is the value of the last statement in it)

## Conditionals
- `[x] | [y] | [z]` - if `x != 0` then `y` else `z`

## Variable/Function Names
- Valid bracket nestings only, e.g. `()`, `(())`, `(()())`, `(()(()))`, etc.

## Examples 
### addition (5+6)
```
{(()())}( () (()) ) [
    [ {(())}[] ] | 
        [ {(()())} [ &{()}[] *{(())}[] ] ]
    |
        [ {()}[] ]
]
{(()())}[ [()()()()()] [()()()()()()] ]
```

### multiplication (6*8)
```
{(()())}( () (()) ) [
    [ {(())}[] ] | 
        [ {(()())} [ &{()}[] *{(())}[] ] ]
    |
        [ {()}[] ]
]

{(()()()())} ( () (()) ((())) ) [
    [ {(())}[] ] |
        [ {(()()()())} [ {()}[] *{(())}[] {(()())} [ {((()))}[] {()}[] ] ] ]
    |
        [ {((()))}[] ]
]

{(()()())} ( () (()) ) [
    {(()()()())} [ {()}[] {(())}[] [] ]
]

{(()()())} [ [()()()()()()] [()()()()()()()()]]
```