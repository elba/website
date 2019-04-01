#!/bin/sh

# If the backend is started before postgres is ready, the migrations will fail
until ./diesel database setup; do
  echo "Migrations failed, retrying in 5 seconds..."
  sleep 5
done

exec ./elba-backend
