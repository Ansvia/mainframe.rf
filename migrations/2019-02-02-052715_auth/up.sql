CREATE TABLE access_tokens (
    token TEXT PRIMARY KEY,
    $param.service_name_snake_case$_id BIGINT NOT NULL REFERENCES $param.service_name_snake_case$s (id) ON DELETE CASCADE,
    created TIMESTAMP NOT NULL,
    valid_thru TIMESTAMP NOT NULL
);

CREATE INDEX idx_access_tokens_$param.service_name_snake_case$_id ON access_tokens (
    ($param.service_name_snake_case$_id)
);

