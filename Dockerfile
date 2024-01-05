# 前端静态文件生成
FROM node:lts-alpine as frontend-builder

WORKDIR /frontend
COPY ./src/riot-frontend/package*.json ./

# 淘宝源国内更快
RUN npm config set registry https://registry.npm.taobao.org
RUN npm ci

COPY ./src/riot-frontend .
RUN npm run build

# 后端部署
FROM rust:latest
WORKDIR /app
COPY ./src/riot-backend .
COPY --from=frontend-builder /frontend/dist /app/dist

RUN cargo build --release

EXPOSE 8888

# 请确认参数中的配置文件存在于./src/riot_backend目录下
CMD ["cargo", "run", "--release", "--", "riot_config.prod.toml"]