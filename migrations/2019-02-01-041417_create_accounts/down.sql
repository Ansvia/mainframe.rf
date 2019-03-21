DROP INDEX register_accounts_email;
DROP INDEX register_accounts_phone_num;

DELETE FROM accounts WHERE id = 0;

DROP TABLE account_keys;
DROP TABLE account_passhash;
DROP TABLE register_accounts;
DROP TABLE addresses;
DROP TABLE accounts;


