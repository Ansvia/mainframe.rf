DROP INDEX register_$param.service_name_snake_case$s_email;
DROP INDEX register_$param.service_name_snake_case$s_phone_num;

DELETE FROM $param.service_name_snake_case$s WHERE id = 0;

DROP TABLE $param.service_name_snake_case$_keys;
DROP TABLE $param.service_name_snake_case$_passhash;
DROP TABLE register_$param.service_name_snake_case$s;
DROP TABLE addresses;
DROP TABLE $param.service_name_snake_case$s;


