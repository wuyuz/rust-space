const rust = import('./pkg');

rust
  .then(m => m.greet('Worldsss!'))
  .catch(console.error);