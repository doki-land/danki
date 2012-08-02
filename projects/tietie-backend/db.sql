create table user_info
(
    user_id     bigint                                             not null
        constraint user_info_pk
            primary key,
    user_name   text                                               not null
        constraint user_info_k2
            unique,
    nick_name   text                     default ''::text          not null,
    password    text                     default ''::text          not null,
    create_time timestamp with time zone default CURRENT_TIMESTAMP not null,
    create_user bigint                                             not null,
    update_time timestamp with time zone default CURRENT_TIMESTAMP not null,
    update_user bigint                                             not null,
    type        smallint                 default 0                 not null,
    status      smallint                 default 0                 not null
);

alter table user_info
    owner to postgres;

create function user_create("@user_id" bigint, "@user_name" text, "@nick_name" text, "@password" text) returns void
    language plpgsql
as
$$
BEGIN
    INSERT INTO user_info
    (
        user_id, user_name, nick_name, password, create_time, update_time
    )
    VALUES
        (
            "@user_id", "@user_name", "@nick_name", "@password", current_timestamp, current_timestamp
        );
END;
$$;

alter function user_create(bigint, text, text, text) owner to postgres;

