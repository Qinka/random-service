# Building image for rust
FROM rust:1.40.0-buster as rust-builder

WORKDIR /usr/src/random-service
ADD . .
WORKDIR /usr/src/random-service/random-service-rs
RUN cargo build --release
RUN ls target/release

# Building image for python

FROM python:3.7 as py-builder
WORKDIR /usr/src/random-service
ADD . .
WORKDIR /usr/src/random-service/random-service-py
RUN pip install nuitka flask pyyaml
RUN python -m nuitka --follow-imports --include-plugin-directory=/usr/local/lib/python3.7/site-packages/jinja2/ --standalone random_service

# Final image
FROM debian:buster
LABEL maintainer="Johann Lee <me@qinka.pro>"
ADD .docker/entrypoint.sh /usr/local/bin
# Install random-service-rs
RUN apt-get update && apt-get install -y libssl-dev openssl tini
COPY --from=rust-builder /usr/src/random-service/random-service-rs/target/release/random-service-rs /usr/local/bin/random-service-rs
# Install random-service-py
COPY --from=py-builder /usr/src/random-service/random-service-py/random_service.dist /usr/local/random_service_py
CMD ["entrypoint.sh"]
