FROM oven/bun:1.0

# When building the image the docker-compose volumes aren't set up
# So we'll set up an app folder
WORKDIR /app

# Copy the package.json and package-lock.json into it
COPY package*.json ./

# And install the packages
RUN [ "bun", "install" ]
RUN [ "bun", "--bun", "run", "svelte-kit", "sync" ]