CREATE TABLE user_info (
    user_id                        Bigint                                             NOT NULL
        CONSTRAINT user_info_pk
            PRIMARY KEY,
    user_name                      Text                                               NOT NULL
        CONSTRAINT user_info_pk_2
            UNIQUE,
    nick_name                      Text                     DEFAULT ''::Text          NOT NULL,
    password                       Text                     DEFAULT ''::Text          NOT NULL,
    create_time                    Timestamp With Time Zone DEFAULT current_timestamp NOT NULL,
    create_user                    Bigint                                             NOT NULL,
    update_time                    Timestamp With Time Zone DEFAULT current_timestamp NOT NULL,
    update_user                    Bigint                                             NOT NULL,
    type                           Smallint                 DEFAULT 0                 NOT NULL,
    status                         Smallint                 DEFAULT 0                 NOT NULL
);

ALTER TABLE user_info
    OWNER TO postgres;

