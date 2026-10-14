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
| Validation of the paper's three claims (dispersion, asynchrony, O(1) per step and O(k) total) | [`crates/ccm-experiments/tests/rooted_async_validation.rs`](../../crates/ccm-experiments/tests/rooted_async_validation.rs), run by [`scripts/validate-rooted-async.sh`](../../scripts/validate-rooted-async.sh) |
| The sweep that checks local decisions against the truth | [`crates/ccm-experiments/examples/rooted_async_sweep.rs`](../../crates/ccm-experiments/examples/rooted_async_sweep.rs) |
| The scaling measurements | [`crates/ccm-experiments/examples/rooted_async_scaling.rs`](../../crates/ccm-experiments/examples/rooted_async_scaling.rs) |
| The synchronous P1Tree construction this builds on, unchanged | [`../p1tree.md`](../p1tree.md) |

## Reproduce

```sh
./scripts/validate-rooted-async.sh          # the three claims, PASS/FAIL each (~1 min)
./scripts/validate-rooted-async.sh --full   # also the 8,208-run truth-check sweeps
cargo test -p ccm-experiments --test rooted_async
cargo run --release -p ccm-experiments --example rooted_async_sweep        # synchronous
cargo run --release -p ccm-experiments --example rooted_async_sweep -- 11  # random schedule, seed 11
cargo run --release -p ccm-experiments --example rooted_async_scaling

# The CLI (build with: cargo build --release -p ccm-cli)
./target/release/ccm run    --algorithm rooted-async --scheduler asynchronous --graph grid:4 --nodes 16 --agents 10
./target/release/ccm events --algorithm rooted-async --scheduler asynchronous --graph path --nodes 8 --agents 8
```

In the browser, choose *RootedAsync* from the simulator's algorithm menu,
then *Asynchronous* from the *Schedule* menu that appears below it.

## A2 checklist

- [x] Gap and trace note
- [x] RootedAsync tests
- [x] CLI (Stage 2): `--algorithm rooted-async`, `--scheduler`, `ccm events`
- [x] Web demo (Stage 3): *RootedAsync* in the simulator's menu, with a *Schedule* menu
