FROM rust as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/image-uploader
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
WORKDIR /usr/local/bin
COPY --from=build /usr/local/cargo/bin/image-uploader .
COPY --from=build /usr/src/image-uploader/public ./public
COPY --from=build /usr/src/image-uploader/templates ./templates

CMD ["image-uploader"]