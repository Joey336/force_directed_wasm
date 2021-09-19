// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
import("./index.js").catch(e => console.error("Error importing `index.js`:", e));
const rust = import('../pkg/force_directed');

rust.then(m => {
    m.main_js("0.0");
    document.getElementById('fileContent').addEventListener("DOMSubtreeModified", e=>
    m.main_js(document.getElementById('fileContent').textContent));
  
});

