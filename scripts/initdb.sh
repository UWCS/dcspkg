#! /bin/sh

sqlite3 packagedb.sqlite \
"CREATE TABLE packages(       
    id INTEGER PRIMARY KEY NOT NULL,  
    name STRING NOT NULL,             
    description STRING,               
    version STRING NOT NULL,          
    image_url STRING,                 
    archive_path STRING NOT NULL,     
    executable_path STRING,           
    crc INTEGER NOT NULL,             
    has_installer INTEGER NOT NULL,   
    add_to_path INTEGER NOT NULL,     
    UNIQUE (name, version))"