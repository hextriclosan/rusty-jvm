# Running the Project on a Big-Endian Environment

## Pull and Run the s390x Rust Docker Image
```bash
docker run -it --platform=linux/s390x -v <PROJECT_DIR>:/mnt/rusty-jvm --entrypoint /bin/bash s390x/rust
```

## Download compatible Java Core classes
```shell
apt-get update
apt-get install -y --no-install-recommends curl
export JAVA_HOME="/opt/java"
JAVA_VERSION="23.0.2+7"
JAVA_CHECKSUM="591ccf4d27016315700cc9cc979f7cf343900b6bee1b0b45c93f2c5f946e5aac"
JAVA_MAJOR="23"
ARCH="s390x"
URL="https://github.com/adoptium/temurin${JAVA_MAJOR}-binaries/releases/download/jdk-${JAVA_VERSION}/OpenJDK${JAVA_MAJOR}U-jdk_${ARCH}_linux_hotspot_$(echo ${JAVA_VERSION} | sed 's/+/_/g').tar.gz"
curl -LfsSo /tmp/openjdk.tar.gz "${URL}"
mkdir -p "${JAVA_HOME}"
tar -xzf /tmp/openjdk.tar.gz -C "${JAVA_HOME}" --strip-components=1
```

## Run Integration Tests Inside the Docker Container
```bash
cd /mnt/rusty-jvm/tests/test_data
```
```bash
CARGO_TARGET_DIR=/tmp/target/s390x-unknown-linux-gnu cargo test
```
