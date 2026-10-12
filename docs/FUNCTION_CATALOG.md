# ATC Algorithm — Function Catalog

## Status and authority

The JSON catalog is a source-inventory baseline, not a normative consensus specification and not proof of correctness. It is bound to source tree `5a5628044ef4daacca943062eac3fd27a2eaec93` on `main` as observed when this catalog was created. It must be regenerated and reconciled against the exact PR HEAD before it is treated as current.

## Files

- `docs/function-catalog.json`: discovered function declarations, source paths/line numbers, source blob SHAs, signature excerpts and explicit contract gaps.
- `schemas/function-catalog.schema.json`: machine-readable JSON Schema.
- `tools/function-catalog/generate.py`: regeneration and validation helper.

## Required contract for every function

Each production function must receive owner-reviewed, specification-backed details for:
1. Preconditions and valid input domain.
2. Postconditions and output semantics.
3. Errors and fail-closed behavior.
4. Side effects and state mutation.
5. Determinism and canonical encoding.
6. Security invariants and adversarial inputs.
7. Unit/property/conformance tests and exact commit SHA + CI Run/Job/Step evidence.

The catalog deliberately leaves these semantic fields `UNSPECIFIED`. A name, signature, doc comment or passing unit test alone does not establish consensus conformance.

## Consensus-specific blockers

Repository docs mark consensus specifications ATC-CONSENSUS-301..307 as draft and the consensus engine as a development skeleton. Do not silently resolve conflicts between documented SHA-256 PoH, FNV-1a MVP sequencing and the TownHash development component. Hash algorithm, PoH encoding, validator weighting, overflow behavior, fork choice, finality, PoW difficulty and hybrid-selection semantics require explicit specification decisions and conformance vectors before normative status.

## Scope limitations

This initial snapshot inventories explicit Rust function declarations in `src/**/*.rs` and Python function declarations in discovered Python files, including tests. Macro-generated Rust functions, dynamically constructed callables, and declarative workflow interfaces may require separate inventories. A full source-tree census and CI verification remain required.
