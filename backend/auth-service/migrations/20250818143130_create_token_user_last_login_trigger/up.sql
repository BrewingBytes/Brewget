-- Your SQL goes here
CREATE OR REPLACE FUNCTION update_last_login_at() RETURNS TRIGGER AS $$ BEGIN
UPDATE users
SET last_login_at = NOW()
WHERE id = NEW.user_id;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER token_insert_update_last_login
AFTER
INSERT ON tokens FOR EACH ROW EXECUTE FUNCTION update_last_login_at();
