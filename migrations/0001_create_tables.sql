create table moment_tags (
    moment_id unsigned integer not null,
    kind char(1) not null,
    name varchar not null
);

create unique index moment_tags__unique_idx on moment_tags (moment_id, kind, lower(name));

create table moments (
    id integer primary key,
    ext_id varchar(256) not null, '',
    ext_url text not null default '',
    img_url text not null default '',
    content text not null,
    cost integer not null default 0,
    created_at timestamp not null default current_timestamp
);

create unique index moments__unique_idx on moments (ext_id, lower(content));
