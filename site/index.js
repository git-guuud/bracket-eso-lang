import initSync, { eval_unwrapped } from './node_modules/eso-lang/eso_lang.js';

async function runCode() {
    await initSync();
    const code = document.getElementById('code').value;
    let res = eval_unwrapped(code);
    const output = document.getElementById('output');
    output.value += res + '\n';
}

document.getElementById('run').addEventListener('click', runCode);

document.getElementById('clear').addEventListener('click', () => {
    const output = document.getElementById('output');
    output.value = '';
});