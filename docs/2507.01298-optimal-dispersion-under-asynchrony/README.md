# Optimal Dispersion Under Asynchrony (arXiv:2507.01298)

Work on Pattanayak, Kshemkalyani, Kumar, Molla and Sharma,
*Optimal Dispersion Under Asynchrony*,
[arXiv:2507.01298](https://arxiv.org/abs/2507.01298), for assignment A2:
`RootedAsync`, the paper's asynchronous rooted dispersion algorithm
(Sections 4-5, Algorithms 2-6).

## In this folder

- [gap-and-trace-note.md](gap-and-trace-note.md): how the algorithm is
  modelled, which decisions in the existing P1Tree consult the global
  ownership index, a hand-worked vacated-node probe, the eleven places the
  pseudocode needed repair, verification, and measurements.

## Elsewhere in the repository

| What | Where |
|---|---|
| The algorithm | [`crates/ccm-algorithms/src/rooted_async/`](../../crates/ccm-algorithms/src/rooted_async/) |
| Its tests, including the paper's Appendix B run | [`crates/ccm-experiments/tests/rooted_async.rs`](../../crates/ccm-experiments/tests/rooted_async.rs) |
| The sweep that checks local decisions against the truth | [`crates/ccm-experiments/examples/rooted_async_sweep.rs`](../../crates/ccm-experiments/examples/rooted_async_sweep.rs) |
| The scaling measurements | [`crates/ccm-experiments/examples/rooted_async_scaling.rs`](../../crates/ccm-experiments/examples/rooted_async_scaling.rs) |
| The synchronous P1Tree construction this builds on, unchanged | [`../p1tree.md`](../p1tree.md) |

## Reproduce

```sh
cargo test -p ccm-experiments --test rooted_async
cargo run --release -p ccm-experiments --example rooted_async_sweep        # synchronous
cargo run --release -p ccm-experiments --example rooted_async_sweep -- 11  # random schedule, seed 11
cargo run --release -p ccm-experiments --example rooted_async_scaling
```

## A2 checklist

- [x] Gap and trace note
- [x] RootedAsync tests
- [ ] CLI (Stage 2)
- [ ] Web demo (Stage 3)
