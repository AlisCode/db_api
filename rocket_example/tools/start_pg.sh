#!/bin/sh

docker run --name db_api_pgsql -e POSTGRES_PASSWORD=passwd -d postgres
