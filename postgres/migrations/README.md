# Naming conventions

## Developments
- All up scripts have to named `V{d}__{description}_up.sql`
  - eg. `V1__add_table1_up.sql`
- All down scripts have to named `V{d}__{description}.down.sql`
  - eg. `V1__add_table1.down.sql`
  - Note the change in `_` to `.`
- This is done to be compatible with `https://crates.io/crates/refinery`

