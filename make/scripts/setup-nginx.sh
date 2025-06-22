DIRECTORY="./.guard"

if command -v nginx >/dev/null 2>&1; then
    echo "NGINX is already installed."
    nginx -v
else
    echo "Installing NGINX... (you may be prompted for your sudo password)"
    sudo apt install nginx

    GREEN='\033[0;32m'
    echo "${GREEN}(NGINX) Install complete."
fi
