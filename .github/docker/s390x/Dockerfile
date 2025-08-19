# Use the official cross image for s390x as the base
FROM ghcr.io/cross-rs/s390x-unknown-linux-gnu:main

# Use arguments to easily update Java version and checksum
# Find releases and checksums at: https://adoptium.net/temurin/releases/
ARG JAVA_VERSION="23.0.2+7"
# The checksum is found in the .sha256.txt file next to the release download
ARG JAVA_CHECKSUM="591ccf4d27016315700cc9cc979f7cf343900b6bee1b0b45c93f2c5f946e5aac"
ARG JAVA_MAJOR="23"

# Set environment variables that will persist in the final image
ENV JAVA_HOME=/opt/java
ENV PATH=$JAVA_HOME/bin:$PATH

# Download, verify, extract, and clean up in a single RUN command to optimize layers
RUN set -eux; \
    \
    # Install dependencies for downloading and extraction
    apt-get update; \
    apt-get install -y --no-install-recommends curl; \
    \
    # Download the JDK binary
    ARCH="s390x"; \
    URL="https://github.com/adoptium/temurin${JAVA_MAJOR}-binaries/releases/download/jdk-${JAVA_VERSION}/OpenJDK${JAVA_MAJOR}U-jdk_${ARCH}_linux_hotspot_$(echo ${JAVA_VERSION} | sed 's/+/_/g').tar.gz"; \
    curl -LfsSo /tmp/openjdk.tar.gz "${URL}"; \
    \
    # Verify the checksum
    echo "${JAVA_CHECKSUM} */tmp/openjdk.tar.gz" | sha256sum -c -; \
    \
    # Create target directory and extract the archive
    mkdir -p "${JAVA_HOME}"; \
    tar -xzf /tmp/openjdk.tar.gz -C "${JAVA_HOME}" --strip-components=1; \
    \
    # Clean up
    rm /tmp/openjdk.tar.gz; \
    apt-get purge -y --auto-remove curl; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*; \
    \
    # Verify installation
    java --version \
    && javac --version
