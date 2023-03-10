DROP TABLE IF EXISTS users CASCADE;

DROP FUNCTION IF EXISTS create_user;
DROP FUNCTION IF EXISTS update_user;
DROP FUNCTION IF EXISTS delete_user;
DROP FUNCTION IF EXISTS delete_user_by_email;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION create_user(IN in_email VARCHAR(255)) 
RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMPTZ
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY INSERT INTO users (email) VALUES (in_email) RETURNING *;
END;
$$;

CREATE OR REPLACE FUNCTION create_or_return_user(IN in_email VARCHAR(255))
RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMPTZ
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY SELECT * FROM users WHERE users.email = in_email;
    IF NOT FOUND THEN
        RETURN QUERY INSERT INTO users (email) VALUES (in_email) RETURNING *;
    END IF;
END;
$$;

CREATE OR REPLACE FUNCTION update_user(
    IN in_id INT,
    IN in_new_email VARCHAR(255),
    IN in_created_at TIMESTAMPTZ
) RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMPTZ
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY UPDATE users AS u SET 
    email = in_new_email, 
    created_at = 
        CASE WHEN in_created_at IS NOT NULL 
            THEN in_created_at
        ELSE u.created_at 
        END
    WHERE u.id = in_id RETURNING *;
END;
$$;

CREATE OR REPLACE FUNCTION delete_user(IN in_id INT)
RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMPTZ
) 
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY DELETE FROM users WHERE users.id = in_id RETURNING *;
END;
$$;

CREATE OR REPLACE FUNCTION delete_user_by_email(IN in_email VARCHAR(255))
RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMPTZ
) 
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY DELETE FROM users WHERE users.email = in_email RETURNING *;
END;
$$;
