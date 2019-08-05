# FOR RUNNING THE TEST SUITE ON CI OR LOCALLY
FROM rust:1.36-slim-stretch

# Install runtimes so we can test the executor works with them.
# https://blog.docker.com/2019/07/intro-guide-to-dockerfile-best-practices/
RUN apt-get update \
    && apt-get install -y --no-install-recommends\
    ruby-full php nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && mv /usr/bin/nodejs /usr/bin/node

# NOTE: This is a very old version of node (v4), but it doesn't matter for the test suite.
