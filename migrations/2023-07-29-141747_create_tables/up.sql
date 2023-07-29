create table posts
(
    id        bigint primary key auto_increment,
    title     varchar(255) not null,
    body      text,
    published bool         not null default false
);
