# syntax=docker/dockerfile:1
FROM node:16
WORKDIR .
COPY . .
RUN yarn
CMD ["yarn", "start"]