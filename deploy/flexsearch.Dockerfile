FROM node:11-alpine

WORKDIR /app

RUN npm install flexsearch-server && \
    cp -r node_modules/flexsearch-server/* .

ENTRYPOINT ["npm", "start", "production"]
