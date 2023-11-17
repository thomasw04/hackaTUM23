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

RUN echo "http://localhost:3000 {" > Caddyfile && \
	echo "  root * /usr/share/caddy" >> Caddyfile && \
	echo "  route /craft* {" >> Caddyfile && \
	echo "    reverse_proxy backend:8000" >> Caddyfile && \
	echo "  }" >> Caddyfile && \
	echo "  file_server" >> Caddyfile && \
	echo "}" >> Caddyfile

EXPOSE 3000
CMD ["caddy", "run", "--config", "Caddyfile", "--adapter", "caddyfile"]
