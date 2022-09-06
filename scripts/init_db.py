import sqlite3

con = sqlite3.connect("packagedb.sqlite")
cur = con.cursor()

# this is the ground source of truth for the database schema
# the struct in server/src/package.rs should reflect this
cur.execute(
    """
CREATE TABLE packages(
    id INTEGER PRIMARY KEY NOT NULL,
    name STRING NOT NULL UNIQUE,
    description STRING,
    version STRING NOT NULL,
    image_url STRING,
    archive_path STRING NOT NULL,
    executable_path STRING,
    crc INTEGER NOT NULL,
    has_installer INTEGER NOT NULL,
    add_to_path INTEGER NOT NULL)
"""
)
