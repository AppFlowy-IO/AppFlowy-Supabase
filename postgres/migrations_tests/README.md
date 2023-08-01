# Provides a test suite for functions and triggers of sql schema

## Requirements
- The following environmental variables needs to be set
```
POSTGRES_DB=somedb
POSTGRES_PORT=5432
POSTGRES_HOST=localhost
POSTGRES_PASSWORD=supersecretpassword
POSTGRES_USER=someuser
```
- psql needs to be installed
- Migrations scripts have to run before this

## Commands Scripts
```
# check env and installed binaries in path
./check

# check creation of workspace upon user creation
./af_user
```

# Test with postgresql docker image
- Start postgresql in docker: `docker run --name some-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres`
- Add following environment variables
```
export PGHOST=localhost
export PGPORT=5432
export PGDB=postgres
export PGUSER=postgres
export PGPASSWORD=password
```
- Run the migration (specify path)
```
psql --host=localhost --username=postgres --port=5432 --file your/path/to/V1__Initial_Up.sql
```
- Run the check `./check`
- Run the trigger test for `af_user`: `./af_user`
- Run the trigger test for `af_collab_update`: `./af_collab_update`
