-- Set params
SET SESSION test.n_accounts = '50';
SET SESSION test.n_pictures = '500';
SET SESSION test.n_likes_min = '0';
SET SESSION test.n_likes_max = '50';
SET SESSION test.n_comments_min = '0';
SET SESSION test.n_comments_max = '100';
SET SESSION test.date_min = '2022-01-01 00:00:00';
SET SESSION test.date_max = '2022-06-11 00:00:00';
SET SESSION test.password_hash = '1e623de0d44a6e7443ab927559c02452';

-- Generate Users
INSERT INTO accounts
SELECT uuid_generate_v4(),
	CONCAT('User', id, '@lolmail.com'), 
	CONCAT('User', id),
	CURRENT_SETTING('test.password_hash'),
	true
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_accounts')::int) as id;

-- Random User function
CREATE FUNCTION random_user()
	RETURNS UUID
	LANGUAGE PLPGSQL
AS
$$
	DECLARE account UUID;
	BEGIN
		SELECT account_id INTO account FROM accounts ORDER BY RANDOM() LIMIT 1;
		RETURN account;
	END
$$;

-- Generate Pictures
INSERT INTO pictures
SELECT uuid_generate_v4(), random_user()
FROM GENERATE_SERIES(1, CURRENT_SETTING('test.n_pictures')::int);
