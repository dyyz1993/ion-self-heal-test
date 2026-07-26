# ion-self-heal-test

Test project for ION self-healing pipeline validation.

## Known issue

`Monitor::process_output` panics on `None` input (calls `.unwrap()` on `Option<String>`).
See GitHub issue #1.
