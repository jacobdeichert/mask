FROM alpine:3.10

RUN apk add --no-cache bash curl ca-certificates jq

COPY run.sh /run.sh

ENTRYPOINT ["/run.sh"]
