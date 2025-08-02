# Running the Project on a Big-Endian Environment

### Pull and Run the s390x Rust Docker Image
```bash
docker run -it --platform=linux/s390x -v <PROJECT_DIR>:/mnt/rusty-jvm --entrypoint /bin/bash s390x/rust
```

### Run Integration Tests Inside the Docker Container
```bash
cd /mnt/rusty-jvm/tests/test_data
```
```bash
CARGO_TARGET_DIR=../../target/s390x-unknown-linux-gnu cargo test
```
