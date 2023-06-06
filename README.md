# print-dis-server

## Development Setup

You need to wire this up to a Postgres server with an empty database. If you have access to the cmdcentral VPN or are on-network, one is provided for you (just ask @bjschafer).
To do so, create a file `.env` with the following contents:

```bash
DATABASE_URL=postgres://username:password@hostname/dbname
```

Then, run it with `cargo run`.

To see what it looks like, go to http://localhost:8080/swagger-ui