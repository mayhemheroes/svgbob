# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest AS builder

# Add source code to the build stage.
ADD . /src
WORKDIR /src

# Compile the fuzzers.
RUN cd packages/svgbob && cargo +nightly fuzz build

# Package stage
FROM ubuntu:latest

# Copy the corpus to the final image.
COPY --from=builder /src/packages/svgbob/fuzz/corpus/ /corpus/

# Copy the compiled fuzzers to the final image.
COPY --from=builder /src/packages/svgbob/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/
