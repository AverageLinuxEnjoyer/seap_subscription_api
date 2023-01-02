DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS subscriptions CASCADE;
DROP TABLE IF EXISTS title_keywords; 
DROP TABLE IF EXISTS desc_keywords; 
DROP TABLE IF EXISTS additional_info_keywords; 

DROP FUNCTION IF EXISTS create_user;
DROP FUNCTION IF EXISTS update_user;
DROP FUNCTION IF EXISTS delete_user;

DROP FUNCTION IF EXISTS create_subscription;
DROP FUNCTION IF EXISTS update_subscription;
DROP FUNCTION IF EXISTS get_subscriptions;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION create_user(IN in_email VARCHAR(255)) 
RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMP
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY INSERT INTO users (email) VALUES (in_email) RETURNING *;
END;
$$;

CREATE OR REPLACE FUNCTION update_user(
    IN in_id INT,
    IN in_new_email VARCHAR(255),
    IN in_created_at TIMESTAMP
) RETURNS TABLE (
    id INT,
    email VARCHAR(255),
    created_at TIMESTAMP
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
    created_at TIMESTAMP
) 
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY DELETE FROM users WHERE users.id = in_id RETURNING *;
END;
$$;

CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    id_user INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    min_price INT,
    max_price INT
);

CREATE TABLE title_keywords (
    id_subscription INT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    keyword VARCHAR(50),
    PRIMARY KEY (id_subscription, keyword)
);

CREATE TABLE desc_keywords (
    id_subscription INT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    keyword VARCHAR(50),
    PRIMARY KEY (id_subscription, keyword)
);

CREATE TABLE additional_info_keywords (
    id_subscription INT NOT NULL REFERENCES subscriptions(id) ON DELETE CASCADE,
    keyword VARCHAR(50),
    PRIMARY KEY (id_subscription, keyword)
);


CREATE OR REPLACE FUNCTION create_subscription(
    IN in_id_user INT,
    IN in_min_price INT,
    IN in_max_price INT,
    IN in_title_keywords TEXT[],
    IN in_desc_keywords TEXT[],
    IN in_additional_info_keywords TEXT[]
) RETURNS TABLE (
    id INT,
    id_user INT,
    min_price INT,
    max_price INT,
    title_keywords VARCHAR[],
    desc_keywords VARCHAR[],
    additional_info_keywords VARCHAR[]
)
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO subscriptions (id_user, min_price, max_price) VALUES (in_id_user, in_min_price, in_max_price);

    INSERT INTO title_keywords (id_subscription, keyword) 
        SELECT currval('subscriptions_id_seq'), unnest(in_title_keywords);

    INSERT INTO desc_keywords (id_subscription, keyword) 
        SELECT currval('subscriptions_id_seq'), unnest(in_desc_keywords);

    INSERT INTO additional_info_keywords (id_subscription, keyword) 
        SELECT currval('subscriptions_id_seq'), unnest(in_additional_info_keywords);

    RETURN QUERY 
    SELECT * FROM get_subscriptions() AS T WHERE T.id=currval('subscriptions_id_seq');
END;
$$;



CREATE OR REPLACE FUNCTION update_subscription(
    IN in_id INT,
    IN in_id_user INT,
    IN in_min_price INT,
    IN in_max_price INT,
    IN in_title_keywords TEXT[],
    IN in_desc_keywords TEXT[],
    IN in_additional_info_keywords TEXT[]
) RETURNS TABLE (
    id INT,
    id_user INT,
    min_price INT,
    max_price INT,
    title_keywords VARCHAR[],
    desc_keywords VARCHAR[],
    additional_info_keywords VARCHAR[]
)
LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE subscriptions SET 
        id_user = in_id_user,
        min_price = in_min_price,
        max_price = in_max_price
    WHERE subscriptions.id = in_id;

    DELETE FROM title_keywords WHERE title_keywords.id_subscription = in_id;
    DELETE FROM desc_keywords WHERE desc_keywords.id_subscription = in_id;
    DELETE FROM additional_info_keywords WHERE additional_info_keywords.id_subscription = in_id;

    INSERT INTO title_keywords (id_subscription, keyword) 
        SELECT in_id, unnest(in_title_keywords);

    INSERT INTO desc_keywords (id_subscription, keyword) 
        SELECT in_id, unnest(in_desc_keywords);

    INSERT INTO additional_info_keywords (id_subscription, keyword) 
        SELECT in_id, unnest(in_additional_info_keywords);

    RETURN QUERY 
    SELECT * FROM get_subscriptions() AS T WHERE T.id=in_id;
END;
$$;


CREATE OR REPLACE FUNCTION get_subscriptions() RETURNS
TABLE(
    id INT,
    id_user INT,
    min_price INT,
    max_price INT,
    title_keywords VARCHAR[],
    desc_keywords VARCHAR[],
    additional_info_keywords VARCHAR[]
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        s.id,
        s.id_user,
        s.min_price,
        s.max_price,
        (CASE WHEN EXISTS (SELECT 1 FROM title_keywords WHERE id_subscription = s.id)
            THEN ARRAY(SELECT keyword FROM title_keywords WHERE id_subscription = s.id)
        ELSE 
            NULL
        END),
        (CASE WHEN EXISTS (SELECT 1 FROM desc_keywords WHERE id_subscription = s.id)
            THEN ARRAY(SELECT keyword FROM desc_keywords WHERE id_subscription = s.id)
        ELSE
            NULL
        END),
        (CASE WHEN EXISTS (SELECT 1 FROM additional_info_keywords WHERE id_subscription = s.id)
            THEN ARRAY(SELECT keyword FROM additional_info_keywords WHERE id_subscription = s.id)
        ELSE 
            NULL     
        END)
    FROM subscriptions s;
END;
$$ LANGUAGE plpgsql;