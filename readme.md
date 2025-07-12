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
- value of definition of variable is the value it is being set to.
- value of definition of function is 0.

## Conditionals
- `[x] | [y] | [z]` - if `x != 0` then `y` else `z`

## Variable/Function Names
- Valid bracket nestings only, e.g. `()`, `(())`, `(()())`, `(()(()))`, etc.

## Try-Catch
- `<try_block|catch_block>` - if error is encountered while executing `try_block`, execute continues from `catch_block` instead.

## Print
- `<x>` - print x as a number
- `<x|>` - print x as a character
- Value of the statement is the value of `x`

## Examples 
### addition (5+6)
```
/Define a function named (()()) to add two numbers let's say add(x,y)/

{(()())}( () (()) ) [ 
    [ {(())}[] ] |                          /if y is not 0/
        [ {(()())} [ &{()}[] *{(())}[] ] ]  /then return value of add(x+1, y-1)/
    |
        [ {()}[] ]                          /else if y is 0 return x/
]

/Calling the function on 5 and 6/
{(()())}[ [()()()()()] [()()()()()()] ]
```

### multiplication (6*8)
```
/addition function/
{(()())}( () (()) ) [
    [ {(())}[] ] | 
        [ {(()())} [ &{()}[] *{(())}[] ] ]
    |
        [ {()}[] ]
]

/Helper function say mul_help(x,y,acc)/

{(()()()())} ( () (()) ((())) ) [
    [ {(())}[] ] |              /if y is not 0/                                           
        [ {(()()()())} [        /call mul_help(x, y-1, acc+x)/ 
            {()}[]              /x/
            *{(())}[]           /y-1/
            {(()())} [          /add(acc, x)/
                {((()))}[] 
                {()}[] 
                ] 
            ] 
        ]
    |                           /if y is 0/
        [ {((()))}[] ]          /return acc/
]

/Multiplication function say mul(x,y)/
{(()()())} ( () (()) ) [
    {(()()()())} [ {()}[] {(())}[] [] ] /mul_help(x, y, 0)/
]

{(()()())} [ [()()()()()()] [()()()()()()()()]] /mul(6,8)/
```