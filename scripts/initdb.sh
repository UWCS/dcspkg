#! /bin/sh

mkdir -p packages/packages
touch packages/packagedb.sqlite
sqlite3 packages/packagedb.sqlite \
"CREATE TABLE packages(       
    pkgname STRING PRIMARY KEY NOT NULL,             
    fullname STRING NOT NULL,             
    description STRING,               
    image_url STRING,                 
    executable_path STRING,           
    crc INTEGER NOT NULL,             
    has_installer INTEGER NOT NULL,   
    add_to_path INTEGER NOT NULL)
"