FROM alpine:3.7
ARG database_url=postgresql://postgres@postgre_server/$name_snake_case$?sslmode=disable

#ADD http://178.128.219.222/linux-x86_64-musl/$name_snake_case$_server-nightly-latest /usr/bin/$name_snake_case$_server
#ADD ./$name_snake_case$_server-nightly-latest /usr/bin/$name_snake_case$_server
ADD ./$name_snake_case$_server /usr/bin/$name_snake_case$_server
ADD ./start.sh /usr/bin/start_server.sh
#ADD http://178.128.219.222/linux-x86_64-musl/$name_snake_case$_server-nightly /usr/bin/$name_snake_case$_server
RUN chmod +x /usr/bin/$name_snake_case$_server

ENV DATABASE_URL=$database_url

EXPOSE 8080 9090

CMD ["ash", "/usr/bin/start_server.sh"]
