FROM alpine
RUN mkdir -p ./rbin
WORKDIR ./rbin
COPY ./release/main .
RUN ls .
# PORT is set by Heroku
CMD ["./main"]
