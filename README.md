# Github Pages WebAssembly deployment of https://github.com/Joey336/kiss3d_force_directed_graph

[![Build Status](https://app.travis-ci.com/Joey336/force_directed_wasm.svg?branch=main)](https://app.travis-ci.com/Joey336/force_directed_wasm)

# kiss3d_force_directed_graph

## About The Project

This project was built using the [Kiss3d](https://github.com/sebcrozet/kiss3d) graphics library for Rust.
Project compiled with WebAssembly and utilizes Wasm-Bindgen to allow JavaScript and Rust to interact.

TravisCI is utilized for automated deployment of new pushes to the gh-pages branch of the repository

This program takes in graph data through .dot files and uses a force-directed drawing algorithm to simulate physics between graph components.

![Alt Text](https://media1.giphy.com/media/OGpG9ceEJ1PVFAvTv2/giphy.gif?cid=790b7611551df85a6fbad1eb6b8372a5b2f98775e214cde3&rid=giphy.gif&ct=g)


## Try it for yourself

1) Visit the github pages site: https://joey336.github.io/force_directed_wasm/
2) Load in .dot files for visualization (example dot files can be found in /dot_files)

# User Control
left-click drag will rotate the canvas.  
right-click drag will translate the canvas.  
scroll wheel will zoom in/out.  

