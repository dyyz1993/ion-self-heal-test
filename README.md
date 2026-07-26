# ION-self-heal-test

Test project for ION self-healing pipeline validation.

## Known issue

`Monitor::process_output` panics on `None` input (calls `.unwrap()` on `Option<String>`).
See GitHub issue #1.

**Status: Fixed** — the panic has been resolved.
