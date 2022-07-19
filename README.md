# rusty_tenpin

## Install

To run the program (requires simple rust install, see [rust docs](https://www.rust-lang.org/tools/install)), execute the following:

```
cargo run
```

## Known Limitations

* This solution does not correctly handle the extra rolls allowed in the 10th frame. They are currently treated as normal rolls (allowing for additional bonus points and a max score of 330).

## Misc.

Scoring 1,000 games sequentially and concurrently took 26ms and 19ms, respectively, on the first run. Hooray concurrency! However, further runs of the sequential program were somehow optimized and began to take 17ms, whereas the concurrent solution consistently ran in 19ms. 
