postgres=# CREATE USER rust_svc WITH PASSWORD 'qJFPv8N8Ha';
CREATE ROLE
postgres=# GRANT USAGE ON SCHEMA public TO rust_svc;
GRANT
postgres=# GRANT ALL PRIVILEGES ON DATABASE "postgres" to rust_svc;
GRANT

postgres=# GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO rust_svc;
GRANT
postgres=# GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO rust_svc;
GRANT
postgres=#
GRANT ALL PRIVILEGES ON TABLE users TO rust_svc;

GRANT ALL PRIVILEGES ON TABLE bills TO rust_svc;



GRANT ALL PRIVILEGES ON TABLE need TO rust_svc;


GRANT ALL PRIVILEGES ON TABLE roles TO rust_svc;


GRANT ALL PRIVILEGES ON TABLE permits TO rust_svc;
GRANT ALL PRIVILEGES ON TABLE roletype TO rust_svc;

GRANT ALL PRIVILEGES ON TABLE provider TO rust_svc;


GRANT ALL PRIVILEGES ON TABLE billstatus TO rust_svc;


GRANT USAGE, SELECT ON SEQUENCE bills_id_seq TO rust_svc;


GRANT USAGE, SELECT ON SEQUENCE permits_id_seq TO rust_svc;


GRANT USAGE, SELECT ON SEQUENCE need_id_seq TO rust_svc;


GRANT USAGE, SELECT ON SEQUENCE provider_id_seq TO rust_svc;
ERROR:  permission denied for sequence permits_id_seq


need_id_seq


provider_id_seq