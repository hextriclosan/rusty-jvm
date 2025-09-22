# Running the Project on a Big-Endian Environment

## Pull and Run the s390x Rust Docker Image
```bash
docker run -it --platform=linux/s390x -v <PROJECT_DIR>:/mnt/rusty-jvm --entrypoint /bin/bash s390x/rust
```
## Inside the Docker Container
### Download JDK with the Compatible Core Classes
```shell
apt-get update
apt-get install -y --no-install-recommends curl
export JAVA_HOME="/opt/java"
JAVA_VERSION="25+36"
JAVA_CHECKSUM="3e476e40f920ccfb1915d200bc7a1fba9d68c4adcc0358c5968d15613690b915"
JAVA_MAJOR="25"
ARCH="s390x"
URL="https://github.com/adoptium/temurin${JAVA_MAJOR}-binaries/releases/download/jdk-${JAVA_VERSION}/OpenJDK${JAVA_MAJOR}U-jdk_${ARCH}_linux_hotspot_$(echo ${JAVA_VERSION} | sed 's/+/_/g').tar.gz"
curl -LfsSo /tmp/openjdk.tar.gz "${URL}"
mkdir -p "${JAVA_HOME}"
tar -xzf /tmp/openjdk.tar.gz -C "${JAVA_HOME}" --strip-components=1
```

### Run Integration Tests
```bash
export CARGO_TARGET_DIR=/mnt/rusty-jvm/target/s390x-unknown-linux-gnu
mkdir -p $CARGO_TARGET_DIR
cd $CARGO_TARGET_DIR
cargo test
```
