FROM postgres:alpine
ARG database_name=$name_snake_case$

COPY init.sql /docker-entrypoint-initdb.d/10-init.sql

ENV DATABASE_NAME=$database_name

EXPOSE 5432

CMD ["postgres"]

