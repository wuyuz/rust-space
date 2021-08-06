项目简介：使用Rust构建简单的API服务
学习地址：https://github.com/dhruvasagar/url-mapper-rs/tree/main/src
项目创建：cargo new url-mapper-rs

创建DB：
    1、下载sqlx-cli：cargo install sqlx-cli --no-default-features --features postgres
    2、创建DB：DATABASE_URL=postgres://root:123456@localhost/url_mapper_dev?sslmode=disable sqlx database create
    3、创建迁移文件：DATABASE_URL=postgres://root:123456@localhost/url_mapper_dev?sslmode=disable sqlx migrate add creat_rul_maps
    4、编辑迁移文件，迁移