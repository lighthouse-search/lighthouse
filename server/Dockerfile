# Use the official Rust image as the base
FROM --platform=linux/amd64 rust:latest as build-stage
WORKDIR /mindmap-server

LABEL org.opencontainers.image.source=https://github.com/oracularhades/guard

COPY Cargo.lock /mindmap-server/Cargo.lock
COPY Cargo.toml /mindmap-server/Cargo.toml
COPY README.md /mindmap-server/README.md
COPY src /mindmap-server/src
COPY frontend /mindmap-server/frontend
COPY nginx /mindmap-server/nginx
COPY start-webservers.sh /mindmap-server

RUN apt update -y
RUN apt upgrade -y

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /mindmap-server/* && \
    chown -R linuxbrew:linuxbrew /mindmap-server

# Switch to the non-root user
USER linuxbrew
# Set environment variables for Homebrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

# Build frontend via Node docker.

FROM --platform=linux/amd64 node:lts-bookworm-slim as nextjs-build-stage

COPY --from=build-stage /mindmap-server/frontend /mindmap-server/frontend
WORKDIR /mindmap-server/frontend

RUN apt update -y
RUN apt upgrade -y

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /mindmap-server/* && \
    chown -R linuxbrew:linuxbrew /mindmap-server

USER linuxbrew

# Build front-end
RUN npm install
RUN npm run build
WORKDIR /mindmap-server

FROM --platform=linux/amd64 rust:latest as rust-build-stage
COPY --from=build-stage /mindmap-server/ /mindmap-server/
WORKDIR /mindmap-server/

RUN apt update -y
RUN apt upgrade -y

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /mindmap-server/* && \
    chown -R linuxbrew:linuxbrew /mindmap-server

USER linuxbrew

ENV PATH="/home/linuxbrew/.cargo/bin:${PATH}"

# Build
RUN cargo update
RUN cargo build --release

FROM --platform=linux/amd64 rust:latest as production-stage
WORKDIR /mindmap-server

# Copy only compiled Rust/NextJS. No need for source files, they just take up space.
COPY --from=rust-build-stage /mindmap-server /mindmap-server
COPY --from=nextjs-build-stage /mindmap-server/frontend/_static /mindmap-server/frontend/_static

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git build-essential openssl libssl-dev libcap2-bin nginx tree

# Move only relevant files.
RUN mkdir /mindmap-server-built
RUN mkdir /mindmap-server-built/frontend
RUN mv /mindmap-server/frontend/_static /mindmap-server-built/frontend/_static
RUN mv /mindmap-server/target /mindmap-server-built/target
RUN mv /mindmap-server/nginx /mindmap-server-built/nginx
RUN mv /mindmap-server/start-webservers.sh /mindmap-server-built/start-webservers.sh

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /mindmap-server/* && \
    chown -R linuxbrew:linuxbrew /mindmap-server && \
    chown -R linuxbrew:linuxbrew /mindmap-server-built && \
    chown -R linuxbrew:linuxbrew /mindmap-server-built/*

# We'll switch to linuxbrew (dev build account) in-case we do something stupid as root while finalizing this container.
USER linuxbrew

# Copy necessary web-server configuration files for Rover and Guard.
COPY Rocket.toml /mindmap-server/Rocket.toml
RUN mv /mindmap-server/Rocket.toml /mindmap-server-built/Rocket.toml

# Guard
RUN curl -Lo /mindmap-server/guard.zip https://github.com/oracularhades/guard/releases/download/latest/guard.zip
RUN unzip -d /mindmap-server /mindmap-server/guard.zip
# RUN chmod +x /mindmap-server/guard/mindmap-server

# This is about to get moved with the /mindmap-server/guard folder to /mindmap-server-built/guard, so we have no need to move it here.
COPY ./guard/Rocket.toml /mindmap-server/guard/Rocket.toml

COPY guard/guard-config.toml /mindmap-server/guard/guard-config.toml
RUN mv /mindmap-server/guard /mindmap-server-built/guard

# We've filtered out things we don't need, overwrite the original source.
WORKDIR /
USER root
RUN rm -rf /mindmap-server
RUN mv /mindmap-server-built /mindmap-server
WORKDIR /mindmap-server

# Add a non-root user kube with restricted shell
RUN adduser kube --disabled-login
RUN usermod -s /bin/rbash kube
RUN chown -R kube /var/lib/nginx
RUN chown -R kube /var/log/nginx

RUN chmod +x /mindmap-server/guard/guard-server
RUN chmod +x /mindmap-server/start-webservers.sh

EXPOSE 80

# Set the capability to bind to port 80 for the cargo binary
RUN setcap 'cap_net_bind_service=+ep' /usr/sbin/nginx

# Run the application as kube user
USER kube
CMD export guard_config=$(cat /mindmap-server/guard/guard-config.toml) & /mindmap-server/start-webservers.sh