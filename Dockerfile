FROM alpine
RUN mkdir -p ./rbin
WORKDIR ./rbin
COPY ./release/main .
RUN ls .
# PORT is set by Heroku
ENV PORT=8080
EXPOSE ${PORT}
CMD ["./main"]
