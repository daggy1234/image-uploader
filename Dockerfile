FROM rust:1.48.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/dag_cdn
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/dag_cdn .
COPY --from=build /usr/src/dag_cdn/public ./public
COPY --from=build /usr/src/dag_cdn/templates ./templates

CMD ["dag_cdn"]