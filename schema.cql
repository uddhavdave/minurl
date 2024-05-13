create table if not exists urls
(
    uri        text primary key,
    expiration timestamp,
    longurl    text
);

create index if not exists longurlindex
    on urls (longurl);
