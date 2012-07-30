create table user_info
(
    user_id     Blob            not null
        constraint user_info_pk primary key,
    user_name   Text            not null
        constraint user_info_k2 unique,
    nick_name   Text default '' not null,
    email       Text default '' not null,
    password    Text default '' not null,
    create_time Blob            not null,
    update_time Blob
);

