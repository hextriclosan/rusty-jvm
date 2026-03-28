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
JAVA_VERSION="25.0.2+10"
JAVA_CHECKSUM="15e5cbcadcf3d43623c31b825063cdc2817b9f1ba840b51dc6ef70e5d33c84e3"
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
