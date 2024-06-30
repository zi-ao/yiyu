api-dev :
	cd api && cargo watch -x run

api-run :
	cd api && cargo run

num :=
migrate :
	cd api/migration && cargo run -- up $(num)

migrate-rollback :
	cd api/migration && cargo run -- down $(num)

migrate-fresh :
	cd api/migration && cargo run -- fresh

migrate-refresh :
	cd api/migration && cargo run -- refresh

migrate-reset :
	cd api/migration && cargo run -- reset

name :=
migration-generate :
	cd api/migration && cargo run -- generate create_$(name)_table
