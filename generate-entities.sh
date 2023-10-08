#!/bin/sh
docker compose -f docker-compose-generate-entity.yaml up --build --force-recreate
docker compose down