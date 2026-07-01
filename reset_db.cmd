:: Shut down the containers, delete the db volume, and recreate the containers
docker compose down
docker volume rm products_postgres_data
docker compose create
