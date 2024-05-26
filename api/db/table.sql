-- DROP TABLE IF EXISTS problems;
-- CREATE TABLE problems (
--     id VARCHAR(255) PRIMARY KEY,
--     contest_id VARCHAR(255) NOT NULL,
--     problem_index VARCHAR(255) NOT NULL,
--     name VARCHAR(255) NOT NULL,
--     title VARCHAR(255) NOT NULL,
--     platform VARCHAR(255) NOT NULL,
--     point INT,
--     difficulty INT,
--     is_experimental BOOLEAN,
--     url VARCHAR(255) NOT NULL,
--     solver_count INT,
--     submissions INT,
--     success_rate DOUBLE PRECISION,
-- );

DROP TABLE IF EXISTS algorithms;
CREATE TABLE algorithms (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS technical_tags;
CREATE TABLE technical_tags (
    id VARCHAR(255) PRIMARY KEY,
    en_name VARCHAR(255) NOT NULL,
    ja_name VARCHAR(255) NOT NULL,
    algorithm_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (algorithm_id) REFERENCES algorithms(id)
);

