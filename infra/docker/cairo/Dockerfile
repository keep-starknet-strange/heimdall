FROM alpine/git:2.36.3 as cloner
WORKDIR /compiler
RUN git clone https://github.com/starkware-libs/cairo

FROM rust:1.65-alpine3.16 as builder
COPY --from=cloner /compiler/cairo /cairo/
WORKDIR /cairo
RUN apk add alpine-sdk \
    && cargo install --path crates/runner \
    && cargo install --path crates/compiler

FROM alpine:3.16
COPY --from=builder /usr/local/cargo/bin/cairo-compile /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/cairo-run /usr/local/bin/
COPY --from=builder /cairo/corelib/ /cairo/corelib/
