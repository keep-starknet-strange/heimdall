FROM alpine/git:2.36.3 as git
WORKDIR /compiler
RUN git clone https://github.com/starkware-libs/cairo

FROM rust:1.65-alpine3.16 as builder
WORKDIR /cairo
RUN apk add alpine-sdk \
    && cargo install cairo-lang-runner \
    && cargo install cairo-lang-compiler

FROM alpine:3.16
COPY --from=builder /usr/local/cargo/bin/cairo-compile /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/cairo-run /usr/local/bin/
COPY --from=git /compiler/cairo/corelib/ /usr/local/corelib/
