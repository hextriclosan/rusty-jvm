# Use the official cross image for s390x as the base
FROM ghcr.io/cross-rs/s390x-unknown-linux-gnu:main

# ── apt tools ─────────────────────────────────────────────────────────────────
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ── Java 25 (s390x binary, runs via QEMU inside the container) ────────────────
# Find releases and checksums at: https://adoptium.net/temurin/releases/
ARG JAVA_VERSION="25.0.2+10"
# The checksum is found in the .sha256.txt file next to the release download
ARG JAVA_CHECKSUM="15e5cbcadcf3d43623c31b825063cdc2817b9f1ba840b51dc6ef70e5d33c84e3"
ARG JAVA_MAJOR="25"

ENV JAVA_HOME=/opt/java
ENV PATH=$JAVA_HOME/bin:$PATH

RUN set -eux; \
    ARCH="s390x"; \
    URL="https://github.com/adoptium/temurin${JAVA_MAJOR}-binaries/releases/download/jdk-${JAVA_VERSION}/OpenJDK${JAVA_MAJOR}U-jdk_${ARCH}_linux_hotspot_$(echo ${JAVA_VERSION} | sed 's/+/_/g').tar.gz"; \
    curl -LfsSo /tmp/openjdk.tar.gz "${URL}"; \
    echo "${JAVA_CHECKSUM} */tmp/openjdk.tar.gz" | sha256sum -c -; \
    mkdir -p "${JAVA_HOME}"; \
    tar -xzf /tmp/openjdk.tar.gz -C "${JAVA_HOME}" --strip-components=1; \
    rm /tmp/openjdk.tar.gz; \
    java --version \
    && javac --version

# ── Rust stable (x86_64) + s390x cross-compilation target ─────────────────────
ARG RUST_VERSION="stable"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --no-modify-path \
            --default-toolchain "${RUST_VERSION}" \
            --target s390x-unknown-linux-gnu \
            --profile minimal \
    && /root/.cargo/bin/rustup show

ENV PATH="/root/.cargo/bin:${PATH}"
# Tell Cargo to use the s390x cross-linker when targeting s390x
ENV CARGO_TARGET_S390X_UNKNOWN_LINUX_GNU_LINKER=s390x-linux-gnu-gcc
# Tell QEMU where to find the s390x dynamic libraries
ENV QEMU_LD_PREFIX=/usr/s390x-linux-gnu

# ── cargo-nextest (x86_64 binary) ─────────────────────────────────────────────
RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C /usr/local/bin
