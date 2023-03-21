-- Your SQL goes here
CREATE TABLE "BriefUrl" (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       user_name TEXT NOT NULL DEFAULT 'system',
                       url TEXT NOT NULL,
                       short_url TEXT,
                       long_url TEXT,
                       clicks INTEGER NOT NULL DEFAULT 0,
                       click_date TIMESTAMP,
                       active INTEGER NOT NULL DEFAULT 1,
                       date_accessed TIMESTAMP,
                       date_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);