# shannon-entropy-wasm

A rust + webassembly library to calculate the Shannon Entropy of a string.

## Example

``` js
const { default: init, shannon_entropy } = await import('./pkg/entropy.js');
await init();
shannon_entropy('Hello World!');

// 3.0220556259155273
```
