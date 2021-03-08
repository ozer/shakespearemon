<h1 align="center">ShakespeareMon</h1>
<div align="center">
 <strong>
   Translates a pokemon description to Shakespeare
 </strong>
</div>

<br />

ShakespeareMon is a Rest API written in Rust. It listens `/pokemon/{name}` endpoint to get a pokemon name, gets its
description from PokeAPI and then, returns the translation of it by `Shakespeare`.

# Overview

This codebase was created to demonstrate a simple backend application built with Rust and `actix-web`. It includes unit
and integration tests using [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs).

# Getting Started

## Setup

### Prerequisites

- Rust 1.49 (see [here](https://www.rust-lang.org/tools/install) for instructions)

## How to run?

`using cargo`

- ```cargo run```

`using Docker`

- ```docker build -t shakespearemon .```
- ```docker run -p 8080:8080 shakespearemon```

`Testing the endpoint`

- ```curl --location --request GET 'http://localhost:8080/pokemon/pikachu'```

## How to run tests?

- ```cargo test```

## Future Work
- Caching in `Dockerfile` so, it'll take less time at deployment.