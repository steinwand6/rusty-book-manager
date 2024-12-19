INSERT INTO
    roles(name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users(name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$sJV9c1dkikNyYKIT7j81deeexC4HLVn5ZJnpf1s8W9/kmwAfxv8KW',
    role_id
FROM
    roles
where
    name = 'Admin';