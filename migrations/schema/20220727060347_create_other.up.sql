-- Add up migration script here
create table others (
    id bigint auto_increment primary key,
    todo_id bigint,
    title varchar(255),
    description text,
    hoge varchar(255),
    fuga varchar(255),
    don varchar(255),
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp on update current_timestamp,
    deleted_at datetime default null,
    constraint foreign key fk_todo_id (todo_id) references todo(id)
) character set utf8mb4 collate utf8mb4_bin;