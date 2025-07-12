import initSync, { eval_unwrapped } from './node_modules/eso-lang/eso_lang.js';

async function runCode() {
    await initSync();
    const code = document.getElementById('code').value;
    try {
        const output = document.getElementById('output');
        let res = eval_unwrapped(code);
        output.value += "EXITED WITH VALUE: "+ res + '\n';
    }
    catch (e) {
        const output = document.getElementById('output');
        output.value += "Stack overflow: Due to the reliance on recursion + small stack size of wasm, sadly the stack overflows too fast ;(\n";
        // output.value += e.toString() + '\n';
        return;
    }
}

document.getElementById('run').addEventListener('click', runCode);

document.getElementById('clear').addEventListener('click', () => {
    const output = document.getElementById('output');
    output.value = '';
});

let examples = {
    'addition': `
/Define a function named (()()) to add two numbers let's say add(x,y)/

{(()())}( () (()) ) [ 
    [ {(())}[] ] |                          /if y is not 0/
        [ {(()())} [ &{()}[] *{(())}[] ] ]  /then return value of add(x+1, y-1)/
    |
        [ {()}[] ]                          /else if y is 0 return x/
]

/Calling the function on 5 and 6/
{(()())}[ [()()()()()] [()()()()()()] ]
`,
    'multiplication': `/addition function/
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
`,
    'factorial': `
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

/{(()()())} [ [()()()()()()] [()()()()()()()()]]/ /mul(6,8)/

/factorial helper functions say fac_help(x, acc)/
{(()()()()())} ( () (()) ) [
    [ {()}[] ] |           /if x!=0/
        [ {(()()()()())} [ /fac_help(x-1, mul(acc,x))/
            *{()}[]
            {(()()())} [ {(())}[] {()}[] ]
            ]
        ]
    |
        [ {(())}[] ]        /if x=0 acc/
]

/factorial function say fac(x)/
{((()()))} ( () ) [
    {(()()()()())} [ {()}[] [()] ] /fac_help(x,1)/
]

{((()()))} [ [()()()()()()] ] /fac(6) !!DON'T do more than 6 the lang is too unoptimized/
`

};

for (const [key, value] of Object.entries(examples)) {
    const option = document.createElement('option');
    option.value = key;
    option.textContent = key;
    document.getElementById('examples').appendChild(option);
}


document.getElementById('examples').onchange = function() {
    const code = document.getElementById('code');
    code.value = examples[this.value] || '';
};