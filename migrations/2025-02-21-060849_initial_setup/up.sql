CREATE TABLE users (
    google_id VARCHAR(255) UNIQUE PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR UNIQUE NOT NULL,
    role VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    title VARCHAR NOT NULL,
    body TEXT,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    CONSTRAINT fk_task_user FOREIGN KEY (user_id) REFERENCES users(google_id) ON DELETE CASCADE
);

CREATE TABLE dot (
    id SERIAL PRIMARY KEY,
    date INTEGER NOT NULL,
    complete BOOLEAN NOT NULL,
    progress INTEGER,
    task_id INTEGER NOT NULL,
    CONSTRAINT fk_dot_task FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE CASCADE
);
