# [Nx](https://nx.dev) Rust Demo

This is demo repo using Nx to build Rust

There are 2 projects: `server`, and `cats`.

Running `./nx run server:run` will start the server which is written using actix-web.

Running `./nx graph` will show that `server` depends on `cats`.

## .nx Installation

Nx is installed into the `.nx` directory with some entrypoints at the root (`nx` and `nx.bat`). This is so rust developers can easily work in this repo without having to running `npm install` or anything non-Rust related.

[Learn more on nx.dev](https://nx.dev/more-concepts/nx-and-the-wrapper#nx-wrapper)

