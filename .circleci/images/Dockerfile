# FOR RUNNING THE TEST SUITE ON CI OR LOCALLY
FROM rust:1.36-slim-stretch

# Install runtimes so we can test the executor works with them.
# https://blog.docker.com/2019/07/intro-guide-to-dockerfile-best-practices/
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ruby-full php curl \
    && curl -sL https://deb.nodesource.com/setup_12.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

# Node installation instructions
# https://github.com/nodesource/distributions
