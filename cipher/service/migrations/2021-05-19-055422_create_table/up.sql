-- Your SQL goes here
CREATE DATABASE IF NOT EXISTS prop DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

CREATE TABLE code
(
    id         BINARY(12)  NOT NULL,
    email      VARCHAR(64) NOT NULL,
    value      MEDIUMINT   NOT NULL,
    type       TINYINT     NOT NULL DEFAULT 0,
    ip         VARBINARY(16),
    used_at    TIMESTAMP,
    created_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP   NOT NULL,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX code_idx_ip_created_at ON code (ip, created_at);
CREATE INDEX code_idx_email_created_at ON code (email, created_at);

CREATE TABLE user
(
    id         BINARY(12)      NOT NULL,
    email      VARCHAR(64)     NOT NULL,
    level      TINYINT         NOT NULL DEFAULT 0,
    expired_at TIMESTAMP       NOT NULL,
    settings   VARBINARY(5120) NOT NULL,
    created_at TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    enabled    TINYINT         NOT NULL DEFAULT 1,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE UNIQUE INDEX user_idx_email ON user (email);

CREATE TABLE token
(
    id         BINARY(12) NOT NULL,
    user_id    BINARY(12) NOT NULL,
    a          BINARY(48) NOT NULL,
    r          BINARY(48) NOT NULL,
    platform   TINYINT    NOT NULL DEFAULT 0,
    device_id  BINARY(12) NOT NULL,
    expired_at TIMESTAMP  NOT NULL,
    created_at TIMESTAMP  NOT NULL DEFAULT current_timestamp,
    enabled    TINYINT    NOT NULL DEFAULT 1,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX token_idx_user_id_device_id ON token (user_id, device_id);
CREATE INDEX token_idx_user_id_enabled_created_at ON token (user_id, enabled, created_at);

CREATE TABLE login
(
    id                  BINARY(12) NOT NULL,
    user_id             BINARY(12) NOT NULL,
    platform            TINYINT    NOT NULL DEFAULT 0,
    ip                  VARBINARY(16),
    device_id           BINARY(12) NOT NULL,
    client_name         VARCHAR(64),
    client_version_name VARCHAR(64),
    client_version_code INT,
    host_name           VARCHAR(64),
    host_version        VARCHAR(64),
    os_name             VARCHAR(64),
    os_version          VARCHAR(64),
    created_at          TIMESTAMP  NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX login_idx_user_id_created_at ON login (user_id, created_at);

CREATE TABLE item
(
    id         BINARY(12)       NOT NULL,
    user_id    BINARY(12)       NOT NULL,
    type       TINYINT          NOT NULL DEFAULT 0,
    title      VARCHAR(128),
    website    VARCHAR(128),
    x_alg      TINYINT          NOT NULL DEFAULT 0,
    x          VARBINARY(10240) NOT NULL,
    created_at TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP        NOT NULL,
    deleted_at TIMESTAMP,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX item_idx_user_id_updated_at ON item (user_id, updated_at ASC);

CREATE TABLE `order`
(
    id           BINARY(12)      NOT NULL,
    user_id      BINARY(12)      NOT NULL,
    level        TINYINT         NOT NULL DEFAULT 0,
    plan         VARBINARY(2048) NOT NULL,
    quantity     SMALLINT        NOT NULL,
    total_price  INT             NOT NULL,
    platform     TINYINT         NOT NULL DEFAULT 0,
    payment_type TINYINT         NOT NULL DEFAULT 0,
    payment_id   VARCHAR(128),
    payment_at   TIMESTAMP,
    expired_at   TIMESTAMP       NOT NULL,
    created_at   TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status       TINYINT         NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX order_idx_user_id_updated_at ON `order` (user_id, updated_at DESC);

CREATE TABLE version
(
    id           BINARY(12) NOT NULL,
    platform     TINYINT    NOT NULL DEFAULT 0,
    name         VARCHAR(64),
    version_name VARCHAR(64),
    version_code INT,
    file_path    VARCHAR(256),
    changelog    VARCHAR(512),
    optional     BOOLEAN             DEFAULT FALSE,
    created_at   TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX app_version_idx_platform_optional_version_code ON version (platform, optional, version_code);

CREATE TABLE kv
(
    `key`      VARCHAR(128),
    value      VARBINARY(512) NOT NULL DEFAULT '{}',
    flag       INT            NOT NULL DEFAULT 0,
    created_at TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`key`)
) ENGINE = INNODB;

CREATE TABLE feedback
(
    id         BINARY(12) NOT NULL,
    user_id    BINARY(12) NOT NULL,
    content    VARCHAR(1024),
    created_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
) ENGINE = INNODB;