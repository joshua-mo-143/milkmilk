## Milkmilk: Bootstrap fullstack web applications quickly.
### Introduction
Milkmilk.rs is a crate for bootstrapping Rust web service applications quickly and provides premade routes for you to work with so you don't have to keep making the same 5-6 routes.

This crate will also automatically generate a base Dockerfile for you so that you can deploy quickly.

Currently, only Next.js is supported due to being able to provide easy SSR. Other front ends may get added on in the future.

Axum is the only Rust web framework supported at the moment, but more may be added depending on.

### Pre-requisites
You'll want Rust installed, as well as Node if you want to use a front end. You will also probably want Docker if you plan on deploying to a webservice that requires Docker image deployment.


### Usage
Currently this crate can't be installed via Cargo but you can build it for yourself by running this command:
```
  cargo build --release
```

Then when you're done, you can simply put the executable wherever you'd like and execute the file followed by the "start" arg to bootstrap an application!

Once you're done, you will probably want to do the following things before anything else:

* Set up your migrations (currently this app bootstraps with `SQLx` so you'll want to use [`sqlx-cli`](https://lib.rs/crates/sqlx-cli) to get started).

* Make sure everything else you need is in order, like Nginx or any other alternative forms of storage you might need.

Then you can get started! It's as easy as that. The default database URL set in the .env will be a localhost Postgres database on default connection settings.
  
Currently no flag/values are supported, but this will probably change in future. 

### Contact

You can find me at my [twitter](https://www.twitter.com/joshmo_dev).