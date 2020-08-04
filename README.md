# Fractal Flames

## About
Implementation in Rust and WASM of fractal flames based on [The fractal flame algorithm (Draves and Reckase, 2003)](https://flam3.com/flame_draves.pdf).

Any feedback, comments or tips are welcome.
This is just my first attempt at Rust and WASM.
The code will most likely slowly evolve as I learn more about these technologies.

## Setup
You'll need to install `wasm-pack` and `nmp`.

You can compile the source using the following command
```
wasm-pack build
```

In the `www` directory, you'll first need to run
```
npm install
```
And then you can run the server to try the code in `localhost:8080`
```
npm run start
```

## Parameters
In the source code, you can set
- the size of the canvas,
- the number of iteration (it is always greater or equal to the number of actual points drawn in the image as points that fall out of the canvas are discarded),
- the number of functions.

You can also manually pic the variation that will be used to compose the functions.
