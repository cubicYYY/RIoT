CREATE DATABASE IF NOT EXISTS `riot`;
USE `riot`;
CREATE TABLE IF NOT EXISTS `user` (
    `id` SERIAL PRIMARY KEY,
    `username` VARCHAR(64) NOT NULL,
    `email` VARCHAR(256),
    `password` VARCHAR(64),
    `privilege` INT,
    `api_key` VARCHAR(64),
    `since` DATETIME(3),
    `activated` BOOLEAN DEFAULT FALSE,
    INDEX username_index (`username`),
    INDEX api_index (`api_key`)
);
CREATE TABLE IF NOT EXISTS `device` (
    `id` SERIAL PRIMARY KEY,
    `uid` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(255),
    `desc` TEXT,
    `dtype` INT,
    `since` DATETIME(3),
    `last_update` DATETIME(3),
    `activated` BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (`uid`) REFERENCES `user`(id)
);
CREATE TABLE IF NOT EXISTS `site` (
    `id` SERIAL PRIMARY KEY,
    `uid` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(255),
    `desc` TEXT,
    `activated` BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (`uid`) REFERENCES `user`(id)
);

CREATE TABLE IF NOT EXISTS `record` (
    `id` SERIAL PRIMARY KEY,
    `did` BIGINT UNSIGNED NOT NULL,
    `payload` BINARY,
    `latitude` DOUBLE,
    `longitude` DOUBLE,
    `timestamp` DATETIME(3),
    FOREIGN KEY (`did`) REFERENCES `device`(id)
);

CREATE TABLE IF NOT EXISTS `owns` (
    `sid` BIGINT UNSIGNED NOT NULL,
    `did` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (`sid`, `did`),
    FOREIGN KEY (`sid`) REFERENCES `site`(id),
    FOREIGN KEY (`did`) REFERENCES `device`(id)
);
