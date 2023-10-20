FROM docker.io/oven/bun:1.0 as bun

# When building the image the docker-compose volumes aren't set up
# So we'll set up an app folder
WORKDIR /app

FROM bun as install
# Copy the package.json and package-lock.json into it
COPY package*.json ./

# And install the packages
RUN [ "bun", "install" ]

FROM bun as svelte_sync
# Add svelte.config.js and packages
COPY svelte.config.js svelte.config.js
COPY --from=install /app/node_modules /app/node_modules
# Generate .svelte-kit
RUN [ "bun", "--bun", "run", "svelte-kit", "sync" ]

FROM bun as runner
# Add the modules and .svelte-kit
COPY --from=install /app/node_modules /app/node_modules
COPY --from=svelte_sync /app/.svelte-kit /app/.svelte-kit
# Add package*.json
COPY --from=install /app/package*.json /app/
# The volumes in docker-compose.yaml will provide the other needed files