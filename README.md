# Tauri Diesel Example

Uses Tauri v2 Beta

Vanilla js/html frontend with no build step.

`cargo tauri build` should work without any modifications.

To use `cargo tauri dev`, in `src-tauri/tauri-conf.json`, replace `build.beforeDevCommand` with your favorite static server, serving src-www.

Start app with `cargo tauri dev`

<img width="634" alt="Screenshot 2024-03-16 at 12 51 41" src="https://github.com/bpevs/tauri_diesel_example/assets/8182843/989d9c4b-7106-4867-a238-53ebf287b483">

# src-db

Commands used to build `src-db` into its current state (all files not generated by these commands was hand-added):

- `cd src-db`
- `diesel setup`
- `diesel migration generate initial` (where `initial` is the name of the migration)
- *add migration code*
- `diesel migration run`

Other useful commands to know (run in the src-db dir):
- `cargo test`
- `diesel migration redo -n SOME_INT_HERE` if you make changes

Other Notes:
- Notice `.env` in the src-db dir is used for tests. For the app, db location is defined in setup
