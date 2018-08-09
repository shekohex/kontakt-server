FROM alpine
RUN mkdir -p ./rbin
WORKDIR ./rbin
COPY ./release/main .
RUN ls .
ENV PORT=8080
# RUN apk add --no-cache mysql-client
ENTRYPOINT ["./main"]
