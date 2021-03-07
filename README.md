<h1 align="center">ShakespeareMon</h1>
<div align="center">
 <strong>
   Translates a pokemon name to Shakespeare
 </strong>
</div>

<br />

ShakespeareMon is a Rest API written in Rust. It listens `/pokemon/{name}` endpoint to get a pokemon name, 
returns the translation of it by `Shakespeare` if a `pokemon` with given name exists.

# Overview
This codebase was created to demonstrate a simple backend application built with Rust and `actix-web`.
It includes unit and integration tests using [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs).

# Getting Started

## Setup

### Prerequisites

- Rust 1.49 (see [here](https://www.rust-lang.org/tools/install) for instructions)

## How to run?
`using cargo`

- ```cargo run```
- To test the endpoint - ```curl --location --request GET 'http://localhost:8080/pokemon/pikachu'```

## How to run tests?
- ```cargo test```

## Future Work
- As I moved my endpoint handler to other file, ended up with the error below.  
  `App data is not configured, to configure use App::data()`
  
- I've played around Arc, Mutex and done some search however, couldn't fix it.
- After resolving that, I'll continue with the below.
- Organizing the code better in modularized way.
- Moving all the integration tests in `main.src` into `tests` folder, as it is too hard to follow integration tests now :(.