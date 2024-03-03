CREATE TABLE shortened_urls (
    uri text,
    long_uri text,
    expiration timestamp,
    PRIMARY KEY ( uri )
)