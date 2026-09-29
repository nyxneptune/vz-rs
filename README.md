# vz-rs
Implementation of the VZ ordinal notation in Rust. Not very well-optimized yet.

Disclaimer: after the frustration of trying to work it all out myself, I resorted to GPT-6 Astra to work out how the notation should be formalized.
The definition it gives certainly seems to work; see `src/tests.rs` for all the expressions for which it's known to give the right answer.
However, the final product contains no LLM-generated code, it was all implemented by me.
