# Farcaster Indexer

This is a Rust version of [Farcaster-Indexer](https://github.com/gskril/farcaster-indexer)
by [gskril](https://github.com/gskril) which listens for messages
from
a [Farcaster Hub](https://docs.farcaster.xyz/learn/architecture/hubs)
and inserts relevant data into a postgres database.

## How to run

Clone this repo

```bash
git clone https://github.com/ligulfzhou/farcaster-indexer
```

Run the latest database migrations

```bash
cd migration
export DATABASE_URL=postgresql://username:password@localhost/farcaster
cargo run
```

Generate sea-orm entity

```bash 
sea-orm-cli generate entity -u postgresql://username:password@localhost/farcaster -o entity/src
```

Run the indexer

```bash
cd indexer;

# Recommended to get the full state. You only need to run this once.
# Streaming will start after the backfill is complete.
cargo run backfill

# Ignores backfill and start streaming from the latest recorded event.
# You should run this after one initial backfill.
cargo run index
```
