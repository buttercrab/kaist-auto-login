create table if not exists account
(
    no       serial
        constraint account_pk
            primary key,
    email_id varchar(20)  not null,
    email_pw varchar(128) not null,
    id       varchar(128) not null
);

create unique index if not exists account_email_id_index
    on account (email_id);

create unique index if not exists account_id_index
    on account (id);

