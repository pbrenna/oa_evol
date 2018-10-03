Evolving orthogonal arrays
===

Work in progress implementation of: Mariot, L., Picek, S., Jakobovic, D., & Leporati, A. (2018, September). Evolutionary Search of Binary Orthogonal Arrays. In *International Conference on Parallel Problem Solving from Nature* (pp. 121-133). Springer, Cham. 

Requirements
---
 * Rust and related tools (via [rustup](https://rustup.rs/))

Downloading & Running
---
```
$ git clone https://github.com/pbrenna/oa_evol

# To run the genetic algorithm: 
$ cd oa_evol/ga
$ cargo run --release

#To run the genetic programming algorithm
$ cd oa_evol/gp
$ cargo run --release
```