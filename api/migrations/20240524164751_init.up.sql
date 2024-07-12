create extension if not exists pgcrypto with schema public;

set search_path to pg_catalog, public; -- public is necessary so that sqlx can find its _sqlx_migrations table
set plpgsql.extra_warnings to 'all';

create schema iam;

create table iam.user (
    user__id uuid primary key default public.gen_random_uuid(),
    email text not null,
    password text not null,
    first_name text not null,
    last_name text not null,
    created_at timestamptz not null default now(),
    email_verified_at timestamptz,
    last_login timestamptz
);



create table iam.token (
    token__id uuid not null primary key default public.gen_random_uuid(),
    created_at timestamptz not null default statement_timestamp(),
    type text not null,
    revocation_id bytea not null,
    expired_at timestamptz,
    biscuit text,
    user__id uuid ,
    session_id uuid,
    constraint token_type_chk check (type in ('user_access', 'refresh')),
    constraint token_user__id_fk foreign key (user__id) references iam.user (user__id) on delete cascade on update cascade,
    constraint token_user__id_chk check (type not in ('user_access', 'refresh') or user__id is not null),
    constraint token_session_id_chk check (type not in ('user_access', 'refresh') or session_id is not null)
);
