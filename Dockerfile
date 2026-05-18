# syntax=docker/dockerfile:1

FROM node:22-bookworm-slim AS frontend
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

FROM rust:1-bookworm AS backend-build
WORKDIR /app/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY backend/migrations ./migrations
COPY backend/seed ./seed
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=backend-build /app/backend/target/release/gemma-herbalcare-backend /app/gemma-herbalcare-backend
COPY --from=frontend /app/frontend/build /app/static
ENV PORT=8080
ENV STATIC_DIR=/app/static
ENV DATABASE_URL=sqlite:///tmp/gemma_herbalcare.db
EXPOSE 8080
CMD ["/app/gemma-herbalcare-backend"]
