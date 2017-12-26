const assert = require('assert')
const fs = require('fs');
const wasm = fs.readFileSync('./site/reuler.wasm');
console.log(WebAssembly);
assert(WebAssembly, "Web assembly failed to load!");
const lib = WebAssembly.instantiate(toUint8Array(wasm), {});

// Thank you @code_barbarian
function toUint8Array(buf) {
  var u = new Uint8Array(buf.length);
  for (var i = 0; i < buf.length; ++i) {
    u[i] = buf[i];
  }
  return u;
}

const runTest = (number, expected, actual) => {
  console.log(`${number} Euler!`)
  assert.equal(actual, expected, `${number} euler failed`)
}

lib.then(r => {
  const eulers = r.instance.exports;
  console.dir(eulers)
  runTest("First", 233168, eulers.one());
  runTest("Second", 4613732, eulers.two());
  runTest("Third", 6857, eulers.three());
  runTest("Fifth", 232792560, eulers.five());
  runTest("Sixth", 25164150, eulers.six());
  runTest("Seventh", 104743, eulers.seven());
  //console.log(eulers.eight())
  //runTest("Eighth", 23514624000, eulers.eight()); // Differs from native?
  //console.log(eulers.nine())
  //runTest("Ninth", [375, 200, 425], eulers.nine()); // Requires pointer to heap
  //runTest("Tenth", 142913828922, eulers.ten()); // Answer not coercible to i32, i64 fails
});
