CREATE ROLE daksha_rc WITH LOGIN PASSWORD 'daksha_rc';

GRANT ALL PRIVILEGES ON DATABASE daksha_rc TO daksha_rc;

GRANT USAGE ON SCHEMA public TO daksha_rc;
GRANT CREATE ON SCHEMA public TO daksha_rc;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO daksha_rc;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO daksha_rc;
GRANT ALL PRIVILEGES ON ALL FUNCTIONS IN SCHEMA public TO daksha_rc;
