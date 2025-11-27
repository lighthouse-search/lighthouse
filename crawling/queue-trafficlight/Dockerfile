# Use the official Rust image as the base
FROM --platform=linux/amd64 rust:latest as build-stage
WORKDIR /traffic-light

COPY Cargo.lock /traffic-light/Cargo.lock
COPY Cargo.toml /traffic-light/Cargo.toml
COPY README.md /traffic-light/README.md
COPY src /traffic-light/src

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git build-essential openssl libssl-dev libcap2-bin tree

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /traffic-light/* && \
    chown -R linuxbrew:linuxbrew /traffic-light

RUN mkdir /traffic-light-built
RUN chown -R linuxbrew /traffic-light-built

RUN chown -R linuxbrew /traffic-light

# Switch to the non-root user
USER linuxbrew
# Set environment variables for Homebrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

# I don't think this is needed.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /traffic-light

ENV PATH="/home/linuxbrew/.cargo/bin:${PATH}"

# Update crates
RUN cargo update
# Build
RUN cargo build --release

# Copy necessary web-server configuration files for Rover.
COPY Rocket.toml /traffic-light/Rocket.toml

RUN mv /traffic-light/Rocket.toml /traffic-light-built/Rocket.toml
RUN mv /traffic-light/target/release/traffic-light /traffic-light-built/traffic-light

WORKDIR /
USER root
RUN rm -rf /traffic-light
RUN mv /traffic-light-built /traffic-light
WORKDIR /traffic-light
USER linuxbrew

FROM --platform=linux/amd64 rust:latest as production-stage
WORKDIR /traffic-light

COPY --from=build-stage /traffic-light /traffic-light

# Install libcap2-bin for setting capabilities
RUN apt update
RUN apt install -y libcap2-bin default-mysql-client dnsutils tree

# Add a non-root user kube with restricted shell
RUN adduser kube --disabled-login
RUN usermod -s /bin/rbash kube

# Expose port 80 for the web server
EXPOSE 80

# Remove libcap2-bin and clean up apt cache
RUN apt remove -y libcap2-bin
RUN apt autoremove -y
RUN apt clean

RUN tree /traffic-light

# Run the application as kube user
USER kube
CMD /traffic-light/traffic-light