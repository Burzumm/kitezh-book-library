# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:1-slim-bullseye AS builder
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
# copy app files
COPY . .
RUN cargo fetch

# compile app
RUN cargo build --release

###############
## run stage ##
###############
FROM rust:alpine3.14
WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/kitezh-book-library kitezh-book-library

# set user to non-root unless root is required for your app
USER 1001

# indicate what port the server is running on
EXPOSE 8080

# run server
CMD [ "/app/kitezh-book-library" ]