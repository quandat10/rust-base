server:
	cargo watch -q -c -w src/ -x run
docker-compose:
	docker-compose  -f ./deploy/docker-compose.yml up -d
migrate-new:
	sqlx migrate add -r {name}
migrate-up:
	sqlx migrate run
migrate-down:
	sqlx migrate down
