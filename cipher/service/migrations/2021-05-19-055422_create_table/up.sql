-- Your SQL goes here
CREATE TABLE user
(
    id         SERIAL          NOT NULL,
    email      VARCHAR(64)     NOT NULL UNIQUE,
    created_at TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE UNIQUE INDEX user_idx_email ON user (email);

CREATE TABLE code
(
    id         SERIAL  NOT NULL,
    email      VARCHAR(64) NOT NULL,
    value      VARCHAR(64)   NOT NULL,
    used_at    TIMESTAMP,
    created_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP   NOT NULL,
    PRIMARY KEY (id)
) ENGINE = INNODB;


CREATE TABLE item
(
    id         SERIAL           NOT NULL,
    user_id    BINARY(12)       NOT NULL,
    x          VARBINARY(10240) NOT NULL,
    created_at TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP        NOT NULL,
    PRIMARY KEY (id)
) ENGINE = INNODB;

CREATE INDEX item_idx_user_id_updated_at ON item (user_id, updated_at ASC);
