ALTER table iam.user add column a2f_enable_at timestamptz;

CREATE table iam.a2f (
    user__id uuid not null primary key,
    secret integer not null,
    created_at timestamptz not null default now(),
    verified_at timestamptz,
    constraint a2f_user__id_fk foreign key (user__id) references iam.user (user__id) on delete cascade on update cascade
);