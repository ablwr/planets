# where all the planets at?

Available [here](http://bits.ashleyblewer.com/planets/)!

`cargo web start --target=wasm32-unknown-unknown` to get that party started

Visit http://localhost:8000 with your browser.

Deploying is a little hacky, but to do so, I:

`cargo web deploy --target=wasm32-unknown-unknown`  
`cp -R target/deploy/ docs/`  
- set gh-pages to build from the `docs` folder instead of master