ARG NODE_IMAGE=node:19.7-alpine3.17

FROM $NODE_IMAGE AS backend
WORKDIR /app
COPY backend/package.* .
RUN npm install
COPY backend .
RUN npm run build

FROM $NODE_IMAGE AS frontend
WORKDIR /app
COPY frontend/package.* .
RUN npm install
COPY frontend .
RUN npm run build

FROM $NODE_IMAGE
EXPOSE 8080
WORKDIR /app
COPY --from=backend /app/dist .
COPY --from=backend /app/node_modules ./node_modules
COPY --from=frontend /app/build ./public
CMD ["node", "index.js"]
