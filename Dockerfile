FROM amazonlinux:2

# Setup build environment
RUN mkdir -p /build/src && \
    yum update -y && \
    # Add required packages
    yum install -y awscli gcc openssl-devel tree zip && \
    # Install rust with rustup
    curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

# Build environment setting
WORKDIR /build
ENV PATH=/root/.cargo/bin:/usr/sbin:/usr/bin:/sbin:/bin
# Default build command
CMD \
    cargo build --release --target-dir target_al2 && \
    mv target_al2/release/YOUR_PROJECT_NAME bootstrap && \
    strip --strip-all bootstrap && \
    size bootstrap && \
    ldd  bootstrap && \
    zip -9 -j deploy.zip bootstrap