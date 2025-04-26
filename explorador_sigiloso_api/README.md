🧠 Mental Model: “Followers” as Periodic Observers
Think of a follower like a personal or shared agent that tracks a thing over time and stores a log of its changes. In your case:

The "thing" being followed = Bitcoin price, NEAR validator performance, etc.

The follower object = A persistent record that configures what to follow, who owns it, how often it refreshes, and whether it's public or private.

The follower log = Historical snapshots (one per refresh) of the followed entity.


cargo install sqlx-cli --no-default-features --features postgres

# Postgres
DATABASE_URL=postgres://explorador:explorador123@postgres:5432/explorador_db

echo $DATABASE_URL


usa este comando.

```sh
$ DATABASE_URL=postgres://explorador:explorador123@localhost:5432/explorador_db sqlx migrate run
```