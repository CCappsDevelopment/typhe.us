# --- Stage 1: Development & build ---
FROM node:20-slim AS dev
WORKDIR /app

# For dev, ensure devDependencies are installed
ENV NODE_ENV=

COPY . ./

# Expose Vite dev server port for development (5173 is SvelteKit/Vite dev server default)
EXPOSE 5173

# Entrypoint for dev (only used for dev containers, not production)
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]

# --- Stage 2: Production build ---
FROM node:20-slim AS build
WORKDIR /app

COPY . ./
RUN npm run build

# --- Stage 3: Smaller production runtime ---
FROM node:20-slim AS production
WORKDIR /app

ENV NODE_ENV=production

# Copy only the built output and necessary files
COPY --from=build /app ./

# Expose preview server port (SvelteKit uses 4173 by default for preview)
EXPOSE 4173

CMD ["npm", "run", "preview", "--", "--host", "0.0.0.0"]

