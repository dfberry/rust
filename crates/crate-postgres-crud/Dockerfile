# docker build -t my_postgres_db .
# docker run --name my_postgres_db_1 -p 5432:5432 -d my_postgres_db
# psql -h localhost -p 5432 -U postgres -W
# DATABASE_URL=postgres://postgres:postgres@localhost/postgres

# Use the official PostgreSQL image from the Docker Hub
FROM postgres:latest

# Set environment variables for the default user and password
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres

# Expose the default PostgreSQL port
EXPOSE 5432