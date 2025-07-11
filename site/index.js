import initSync, { eval_unwrapped } from './node_modules/eso-lang/eso_lang.js';

async function runCode() {
    await initSync();
    const code = document.getElementById('code').value;
    const output = document.getElementById('output');
    output.innerText = eval_unwrapped(code);

}

document.getElementById('run').addEventListener('click', runCode);