# samskara-lojix-contract

Contract-only crate defining the shared datalog schema between Samskara (pure datalog agent) and Lojix (transpiler agent).

This crate contains NO logic. It provides:

- Rust types (`TranspilerVersion`, `EvalRequest`, `EvalResult`, etc.) that both agents import.
- Canonical `RelationSchema` metadata describing the CozoDB relations.
- CozoScript generation (`create_relations_cozoscript()`) for bootstrapping the shared relations.
- `AI-init.cozo` — a standalone CozoScript file that either agent can load directly.

Do not add business logic, transpilation code, or datalog evaluation here. Those belong in the respective agent crates.

## VCS

Jujutsu (`jj`) is mandatory. Git is the backend only. Always pass `-m` to
`jj` commands.
