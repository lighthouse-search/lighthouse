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
RUN apt install -y curl git build-essential openssl libssl-dev libcap2-bin tree

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /mindmap-server/* && \
    chown -R linuxbrew:linuxbrew /mindmap-server

RUN mkdir /mindmap-server-built
RUN chown -R linuxbrew /mindmap-server-built

RUN chown -R linuxbrew /mindmap-server
RUN chown -R linuxbrew /mindmap-server/frontend

# Switch to the non-root user
USER linuxbrew
# Set environment variables for Homebrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

# I don't think this is needed.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN brew install node

# Build front-end
WORKDIR /mindmap-server/frontend
RUN npm install
RUN npm run build
WORKDIR /mindmap-server

ENV PATH="/home/linuxbrew/.cargo/bin:${PATH}"

RUN curl -Lo /mindmap-server/guard.zip https://github.com/oracularhades/guard/releases/download/latest/guard.zip
RUN unzip -d /mindmap-server /mindmap-server/guard.zip
# RUN chmod +x /mindmap-server/guard/mindmap-server

# Build
RUN cargo update
RUN cargo build --release

# Remove frontend nextjs source files, they just un-necessarily take up lots of space.
RUN mkdir /mindmap-server-built/frontend
RUN mv /mindmap-server/frontend/_static /mindmap-server-built/frontend/_static
RUN mv /mindmap-server/target /mindmap-server-built/target
RUN mv /mindmap-server/nginx /mindmap-server-built/nginx
RUN mv /mindmap-server/start-webservers.sh /mindmap-server-built/start-webservers.sh

# Copy necessary web-server configuration files for Rover and Guard.
COPY Rocket.toml /mindmap-server/Rocket.toml
RUN mv /mindmap-server/Rocket.toml /mindmap-server-built/Rocket.toml

# This is about to get moved with the /mindmap-server/guard folder to /mindmap-server-built/guard, so we have no need to move it here.
COPY ./guard/Rocket.toml /mindmap-server/guard/Rocket.toml

COPY guard/guard-config.toml /mindmap-server/guard/guard-config.toml
RUN mv /mindmap-server/guard /mindmap-server-built/guard

WORKDIR /
USER root
RUN rm -rf /mindmap-server
RUN mv /mindmap-server-built /mindmap-server
WORKDIR /mindmap-server
USER linuxbrew

FROM --platform=linux/amd64 rust:latest as production-stage
WORKDIR /mindmap-server

COPY --from=build-stage /mindmap-server /mindmap-server

# Install libcap2-bin for setting capabilities
RUN apt update
RUN apt install -y libcap2-bin default-mysql-client dnsutils

# Add a non-root user kube with restricted shell
RUN adduser kube --disabled-login
RUN usermod -s /bin/rbash kube

# RUN echo "defaults\nauth on\ntls on\ntls_starttls on\n\naccount smtp\nhost $smtp_host\nport $smtp_port\nfrom $smtp_from_header\nuser $smtp_username\npassword $smtp_password\n\n# Set default\naccount default : smtp" > /home/kube/.msmtprc
# RUN chown -R kube /home/kube/.msmtprc
# RUN chmod 600 /home/kube/.msmtprc

# RUN echo $smtp_password > /home/kube/.msmtp-password.gpg
# RUN chown -R kube /home/kube/.msmtp-password.gpg

# Expose port 80 for the web server
EXPOSE 80

# Remove libcap2-bin and clean up apt cache
RUN apt remove -y libcap2-bin
RUN apt autoremove -y
RUN apt clean

RUN apt-get install nginx -y
RUN chown -R kube /var/lib/nginx
RUN chown -R kube /var/log/nginx

RUN chmod +x /mindmap-server/guard/guard-server
RUN chmod +x /mindmap-server/start-webservers.sh

# Set the capability to bind to port 80 for the cargo binary
RUN setcap 'cap_net_bind_service=+ep' /usr/sbin/nginx

# Run the application as kube user
USER kube
CMD export guard_config=$(cat /mindmap-server/guard/guard-config.toml) & /mindmap-server/start-webservers.sh
