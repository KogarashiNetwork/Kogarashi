FROM node:14.18.2-alpine

WORKDIR /app

COPY package.json yarn.lock ./

RUN apk add curl make g++ --update-cache &&\
    yarn

ENTRYPOINT sh scripts/local.sh
