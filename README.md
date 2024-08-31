
/fullstack-rust-api/
sqlx migrate add -r init
sqlx migrate run
docker compose up -d

/fullstack-rust-api/backend/
cargo watch -q -c -w src/ -x run

