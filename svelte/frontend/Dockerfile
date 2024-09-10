# Stage 1: Build the Svelte app
FROM node:20 AS build

WORKDIR /app

COPY ./package.json ./package-lock.json ./
RUN npm install

COPY . .
RUN npm run build

# Stage 2: Serve the Svelte app
#FROM nginx:alpine

#COPY --from=build /app/.svelte-kit/output/client /usr/share/nginx/html

# Expose the port Vite uses by default
EXPOSE 4173

CMD ["npm", "run", "preview", "--", "--port", "4173", "--host" ]


#EXPOSE 80

#CMD ["nginx", "-g", "daemon off;"]
