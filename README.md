# Ascii Image Generator

## Frontend

The frontend is a vue.js SPA that calls the .wasm version of the rust module.

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

## Rust Module
This Rust application should be able to convert a PNG or JPG image into ascii text. 

### Usage
```
cargo run input.png/jpg output.txt x_modifier y_modifier
```

### Plan
I plan to use this as a WASM package to run in a webapp that will do the same.

## Wasm learning

Space for learning how to compile wasm.