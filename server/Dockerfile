# Use the official Rust image as the base
FROM --platform=linux/amd64 rust:latest as build-stage
WORKDIR /rover-server

LABEL org.opencontainers.image.source=https://gitlab.com/oracularhades/guard

COPY Cargo.lock /rover-server/Cargo.lock
COPY Cargo.toml /rover-server/Cargo.toml
COPY README.md /rover-server/README.md
COPY src /rover-server/src
COPY frontend /rover-server/frontend
COPY nginx /rover-server/nginx
COPY start-webservers.sh /rover-server

RUN apt update -y
RUN apt upgrade -y
RUN apt install -y curl git build-essential openssl libssl-dev libcap2-bin tree

# Create a non-root user to run Homebrew
RUN useradd -m -s /bin/bash linuxbrew && \
    mkdir -p /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /home/linuxbrew/.linuxbrew && \
    chown -R linuxbrew:linuxbrew /rover-server/* && \
    chown -R linuxbrew:linuxbrew /rover-server

RUN mkdir /rover-server-built
RUN chown -R linuxbrew /rover-server-built

RUN chown -R linuxbrew /rover-server
RUN chown -R linuxbrew /rover-server/frontend

# Switch to the non-root user
USER linuxbrew
# Set environment variables for Homebrew
ENV PATH="/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:$PATH"

# I don't think this is needed.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
RUN brew install node

# Build front-end
WORKDIR /rover-server/frontend
RUN npm install
RUN npm run build
WORKDIR /rover-server

ENV PATH="/home/linuxbrew/.cargo/bin:${PATH}"

RUN curl -Lo /rover-server/guard.zip https://gitlab.com/oracularhades/guard/-/jobs/artifacts/main/raw/guard.zip?job=build_rust_binary
RUN unzip -d /rover-server/guard /rover-server/guard.zip
# RUN chmod +x /rover-server/guard/rover-server

# Update crates
RUN cargo update
# Build
RUN cargo build --release

# Remove frontend nextjs source files, they just un-necessarily take up lots of space.
RUN mkdir /rover-server-built/frontend
RUN mv /rover-server/frontend/_static /rover-server-built/frontend/_static
RUN mv /rover-server/target /rover-server-built/target
RUN mv /rover-server/nginx /rover-server-built/nginx
RUN mv /rover-server/start-webservers.sh /rover-server-built/start-webservers.sh

# Copy necessary web-server configuration files for Rover and Guard.
COPY Rocket.toml /rover-server/Rocket.toml
RUN mv /rover-server/Rocket.toml /rover-server-built/Rocket.toml

# This is about to get moved with the /rover-server/guard folder to /rover-server-built/guard, so we have no need to move it here.
COPY ./guard/Rocket.toml /rover-server/guard/Rocket.toml

COPY guard/guard-config.toml /rover-server/guard/guard-config.toml
RUN mv /rover-server/guard /rover-server-built/guard

WORKDIR /
USER root
RUN rm -rf /rover-server
RUN mv /rover-server-built /rover-server
WORKDIR /rover-server
USER linuxbrew

FROM --platform=linux/amd64 rust:latest as production-stage
WORKDIR /rover-server

COPY --from=build-stage /rover-server /rover-server

# Install libcap2-bin for setting capabilities
RUN apt update
RUN apt install -y libcap2-bin default-mysql-client dnsutils tree

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

RUN chmod +x /rover-server/guard/guard-server
RUN chmod +x /rover-server/start-webservers.sh

# Set the capability to bind to port 80 for the cargo binary
RUN setcap 'cap_net_bind_service=+ep' /usr/sbin/nginx

RUN tree /rover-server

# Run the application as kube user
USER kube
CMD export guard_config=$(cat /rover-server/guard/guard-config.toml) & /rover-server/start-webservers.sh