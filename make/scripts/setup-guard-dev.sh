DIRECTORY="./.guard"

if [ ! -d "$DIRECTORY" ]; then
    # Create a directory for an embedded Guard.
    mkdir $DIRECTORY

    # Download Guard
    echo "Downloading Guard binary..."
    curl -Lo ./.guard/guard.zip https://gitlab.com/oracularhades/guard/-/jobs/artifacts/nightly/raw/guard.zip?job=build_rust_binary

    # Unzip Guard's binary and compiled static frontend.
    echo "Extracting Guard binary..."
    unzip -d ./.guard -o ./.guard/guard.zip

    # Clean-up Guard download
    rm ./.guard/guard.zip

    # Make the Guard binary an executable.
    chmod +x ./.guard/guard-server

    # Run Guard
    GREEN='\033[0;32m'
    echo "${GREEN}(Guard) Install complete. Run Guard using: ./.guard/guard-server"
fi