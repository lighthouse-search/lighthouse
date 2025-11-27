# Use the official Rust image as the base
FROM --platform=linux/amd64 rust:latest as build-stage
WORKDIR /rover-server

LABEL org.opencontainers.image.source=https://gitlab.com/oracularhades/guard

COPY server/Cargo.lock /rover-server/Cargo.lock
COPY server/Cargo.toml /rover-server/Cargo.toml
COPY server/start-webservers.sh /rover-server/start-webservers.sh
COPY server/Rocket.toml /rover-server/Rocket.toml
COPY server/guard /rover-server/guard
COPY server/nginx /rover-server/nginx
COPY server/src /rover-server/src
COPY frontend /rover-server/frontend
COPY libraries /rover-server/libraries

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git build-essential openssl libssl-dev libcap2-bin tree

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /rover-server/* && \
    chown -R linuxbrew:linuxbrew /rover-server

RUN chown -R linuxbrew /rover-server
RUN chown -R linuxbrew /rover-server/frontend

# Switch to the non-root user
USER linuxbrew
# Set environment variables for Homebrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

FROM --platform=linux/amd64 node:latest as build-frontend-stage
WORKDIR /rover-server
COPY --from=build-stage /rover-server/frontend/lighthouse /rover-server/frontend/lighthouse
COPY --from=build-stage /rover-server/libraries /rover-server/libraries

# Build user frontend
WORKDIR /rover-server/frontend/lighthouse
RUN yarn
RUN npm run build

FROM --platform=linux/amd64 rust:latest as build-finalise-stage
COPY --from=build-stage /rover-server /rover-server
COPY --from=build-frontend-stage /rover-server/frontend/lighthouse /rover-server/frontend/lighthouse

WORKDIR /rover-server

RUN curl -Lo /rover-server/guard.zip https://github.com/lighthouse-search/guard/releases/latest/download/guard.zip
RUN unzip -d /rover-server/guard /rover-server/guard.zip
# RUN chmod +x /rover-server/guard/rover-server

# Update crates
RUN cargo update
# Build server
RUN cargo build --release

# Remove frontend nextjs source files, they just un-necessarily take up lots of space.
RUN mkdir /rover-server-built
RUN mkdir /rover-server-built/frontend
RUN mkdir /rover-server-built/frontend/lighthouse
RUN mkdir /rover-server-built/frontend/admin
RUN mv /rover-server/frontend/lighthouse/_static /rover-server-built/frontend/lighthouse/_static
RUN mv /rover-server/frontend/lighthouse/package.json /rover-server-built/frontend/lighthouse/package.json
RUN mv /rover-server/frontend/lighthouse/package-lock.json /rover-server-built/frontend/lighthouse/package-lock.json
RUN mv /rover-server/frontend/lighthouse/next.config.js /rover-server-built/frontend/lighthouse/next.config.js
RUN mv /rover-server/frontend/lighthouse/jsconfig.json /rover-server-built/frontend/lighthouse/jsconfig.json
RUN mv /rover-server/frontend/lighthouse/node_modules /rover-server-built/frontend/lighthouse/node_modules

# Files like opensearch.xml are used in NGINX
RUN mv /rover-server/frontend/lighthouse/public /rover-server-built/frontend/lighthouse/public

# RUN mv /rover-server/frontend/admin/_static /rover-server-built/frontend/admin/_static
RUN mv /rover-server/target /rover-server-built/target
RUN mv /rover-server/start-webservers.sh /rover-server-built/start-webservers.sh
RUN mv /rover-server/Rocket.toml /rover-server-built/Rocket.toml
RUN mv /rover-server/nginx /rover-server-built/nginx

COPY server/guard/guard-config.toml /rover-server/guard/guard-config.toml
RUN mv /rover-server/guard /rover-server-built/guard

WORKDIR /
USER root
RUN rm -rf /rover-server
RUN mv /rover-server-built /rover-server
WORKDIR /rover-server
USER linuxbrew

FROM --platform=linux/amd64 rust:latest as production-stage
WORKDIR /rover-server

COPY --from=build-finalise-stage /rover-server /rover-server

# Install libcap2-bin for setting capabilities
RUN apt update
RUN apt install -y libcap2-bin default-mysql-client dnsutils tree nginx

# Add a non-root user kube with restricted shell
RUN adduser linuxbrew --disabled-login
RUN usermod -s /bin/rbash linuxbrew

USER linuxbrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:/usr/sbin:/usr/bin:/sbin:/bin:$PATH"
RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN brew install node
USER root

RUN chown -R linuxbrew /var/lib/nginx
RUN chown -R linuxbrew /var/log/nginx
RUN mkdir -p /rover-server/nginx/run
RUN chown -R linuxbrew /rover-server/nginx/run

# Expose port 80 for NGINX
EXPOSE 80

RUN setcap 'cap_net_bind_service=+ep' /usr/sbin/nginx

RUN chmod +x /rover-server/guard/guard-server
RUN chmod +x /rover-server/start-webservers.sh

RUN tree /rover-server

# Run the application as linuxbrew user
USER linuxbrew
CMD export guard_config=$(cat /rover-server/guard/guard-config.toml) & /rover-server/start-webservers.sh