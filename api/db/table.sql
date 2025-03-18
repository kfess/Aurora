DROP TABLE IF EXISTS problem_tags;

DROP TABLE IF EXISTS technical_tags;

DROP TABLE IF EXISTS algorithms;

DROP TABLE IF EXISTS contest_problems;

DROP TABLE IF EXISTS problems;

DROP TABLE IF EXISTS contests;

DROP TABLE IF EXISTS internal_users;

CREATE TABLE
    problems (
        id VARCHAR(255) PRIMARY KEY,
        contest_id VARCHAR(255) NOT NULL,
        contest_name VARCHAR(255) NOT NULL,
        problem_index VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        title VARCHAR(255) NOT NULL,
        platform VARCHAR(255) NOT NULL,
        raw_point DOUBLE PRECISION,
        difficulty DOUBLE PRECISION,
        category VARCHAR(255) NOT NULL,
        is_experimental BOOLEAN,
        url VARCHAR(255) NOT NULL,
        solver_count INT,
        submissions INT,
        success_rate DOUBLE PRECISION
    );

CREATE TABLE
    algorithms (
        id VARCHAR(255) PRIMARY KEY,
        name VARCHAR(255) NOT NULL
    );

CREATE TABLE
    technical_tags (
        id VARCHAR(255) PRIMARY KEY,
        en_name VARCHAR(255) NOT NULL,
        ja_name VARCHAR(255) NOT NULL,
        algorithm_id VARCHAR(255) NOT NULL,
        FOREIGN KEY (algorithm_id) REFERENCES algorithms (id)
    );

CREATE TABLE
    problem_tags (
        problem_id VARCHAR(255) NOT NULL,
        technical_tag_id VARCHAR(255) NOT NULL,
        PRIMARY KEY (problem_id, technical_tag_id),
        FOREIGN KEY (problem_id) REFERENCES problems (id),
        FOREIGN KEY (technical_tag_id) REFERENCES technical_tags (id)
    );

CREATE TABLE
    contests (
        id VARCHAR(255) PRIMARY KEY,
        raw_id VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        category VARCHAR(255) NOT NULL,
        platform VARCHAR(255) NOT NULL,
        phase VARCHAR(255) NOT NULL,
        start_time_seconds BIGINT,
        duration_seconds BIGINT,
        url VARCHAR(255) NOT NULL
    );

CREATE TABLE
    contest_problems (
        contest_id VARCHAR(255) NOT NULL,
        problem_id VARCHAR(255) NOT NULL,
        PRIMARY KEY (contest_id, problem_id),
        FOREIGN KEY (contest_id) REFERENCES contests (id),
        FOREIGN KEY (problem_id) REFERENCES problems (id)
    );

CREATE TABLE
    internal_users (
        id VARCHAR(255) PRIMARY KEY,
        username VARCHAR(255),
        github_id VARCHAR(255),
        github_username VARCHAR(255),
        google_id VARCHAR(255),
        google_username VARCHAR(255),
        atcoder_username VARCHAR(255),
        codeforces_username VARCHAR(255),
        yukicoder_username VARCHAR(255),
        aoj_username VARCHAR(255),
        yoj_username VARCHAR(255)
    );

-- Index