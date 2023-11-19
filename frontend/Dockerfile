FROM node:18 AS build

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# Main production container
FROM caddy:2
WORKDIR /usr/share/caddy
COPY --from=build /app/dist .

COPY Caddyfile /etc/caddy/Caddyfile

EXPOSE 5000
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile", "--adapter", "caddyfile"]
