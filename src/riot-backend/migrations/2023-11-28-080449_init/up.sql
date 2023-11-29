CREATE DATABASE IF NOT EXISTS `riot`;
USE `riot`;
CREATE TABLE IF NOT EXISTS `user` (
    `id` SERIAL PRIMARY KEY,
    `username` VARCHAR(64) NOT NULL UNIQUE,
    `email` VARCHAR(256) NOT NULL UNIQUE,
    `password` VARCHAR(256) NOT NULL,
    `privilege` INT UNSIGNED DEFAULT 0 NOT NULL,
    `api_key` VARCHAR(64) DEFAULT NULL,
    `since` DATETIME(3) NOT NULL,
    `activated` BOOLEAN DEFAULT FALSE NOT NULL,
    INDEX username_index (`username`),
    INDEX email_index (`email`),
    INDEX api_index (`api_key`)
);
CREATE TABLE IF NOT EXISTS `device` (
    `id` SERIAL PRIMARY KEY,
    `uid` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `desc` TEXT DEFAULT NULL,
    `dtype` INT UNSIGNED DEFAULT 0 NOT NULL,
    `since` DATETIME(3) NOT NULL,
    `last_update` DATETIME(3) NOT NULL,
    `activated` BOOLEAN DEFAULT FALSE NOT NULL,
    FOREIGN KEY (`uid`) REFERENCES `user`(id)
);
CREATE TABLE IF NOT EXISTS `site` (
    `id` SERIAL PRIMARY KEY,
    `uid` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `desc` TEXT,
    `activated` BOOLEAN DEFAULT FALSE NOT NULL,
    FOREIGN KEY (`uid`) REFERENCES `user`(id)
);

CREATE TABLE IF NOT EXISTS `record` (
    `id` SERIAL PRIMARY KEY,
    `did` BIGINT UNSIGNED NOT NULL,
    `payload` BINARY NOT NULL,
    `latitude` DOUBLE,
    `longitude` DOUBLE,
    `timestamp` DATETIME(3) NOT NULL,
    FOREIGN KEY (`did`) REFERENCES `device`(id)
);

CREATE TABLE IF NOT EXISTS `owns` (
    `sid` BIGINT UNSIGNED NOT NULL,
    `did` BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (`sid`, `did`),
    FOREIGN KEY (`sid`) REFERENCES `site`(id),
    FOREIGN KEY (`did`) REFERENCES `device`(id)
);
