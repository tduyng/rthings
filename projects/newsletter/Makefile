POSTGRES_USER        ?= postgres
POSTGRES_PASSWORD    ?= password
POSTGRES_DB          ?= newsletter
POSTGRES_PORT        ?= 5432
DATABASE_URL         := postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}

export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter

init_db:
	docker run \
		-e POSTGRES_USER=$(POSTGRES_USER) \
		-e POSTGRES_PASSWORD=$(POSTGRES_PASSWORD) \
		-e POSTGRES_DB=$(POSTGRES_DB) \
		-p $(POSTGRES_PORT):5432 \
		-d postgres \
		postgres -N 1000


create_db:
	sqlx database create

migrate_db:
	sqlx migrate add create_subscriptions_table

migrate_run:
	sqlx migrate run

.PHONY: init_db create_db migrate_run
