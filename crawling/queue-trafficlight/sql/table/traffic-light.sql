CREATE TABLE crawler_runner (runner_ip VARCHAR(255) PRIMARY KEY, nonce VARCHAR(255) NOT NULL UNIQUE, capacity BIGINT);
CREATE TABLE crawler_jobs (
    id        VARCHAR(255) NOT NULL,
    runner_ip        VARCHAR(255) NOT NULL,
    nonce            VARCHAR(255) NOT NULL,
    status           TEXT,
    cursor_currently TEXT,
    cursor_from      BIGINT,
    cursor_to        BIGINT,

    /* -------------------------------------------
       Keys & constraints
    ------------------------------------------- */
    PRIMARY KEY (id, runner_ip, nonce),                -- composite PK
    CONSTRAINT fk_jobs_runner_ip
        FOREIGN KEY (runner_ip)
        REFERENCES runner (runner_ip)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT fk_jobs_nonce
        FOREIGN KEY (nonce)
        REFERENCES runner (nonce)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
CREATE TABLE crawler_missingranges (missing_from BIGINT, missing_to BIGINT, PRIMARY KEY (missing_from, missing_to));