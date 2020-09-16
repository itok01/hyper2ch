# Hyper2ch models

## BBS

| id        | name | path_name | description | category | hidden |
| --------- | ---- | --------- | ----------- | -------- | ------ |
| bigserial | text | text      | text        | text     | bool   |

## Thread

| id        | bbs_id    | title | archived | hidden  |
| --------- | --------- | ----- | -------- | ------- |
| bigserial | bigserial | text  | boolean  | boolean |

## Message

| id        | thread_id | user_name | user_email | user_uid | user_ip | user_hostname | user_agent | date       | text | hidden  |
| --------- | --------- | --------- | ---------- | -------- | ------- | ------------- | ---------- | ---------- | ---- | ------- |
| bigserial | bigserial | text      | text       | text     | text    | text          | text       | timestampz | text | boolean |
