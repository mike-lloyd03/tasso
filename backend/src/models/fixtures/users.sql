INSERT INTO users (username, lastname, firstname, email, password_hash, admin)
VALUES (
    'user1',
    'User',
    'Juan',
    'user@email.com',
    '46a9d5bde718bf366178313019f04a753bad00685d38e3ec81c8628f35dfcb1b',
    'f'
),
(
    'userNoPass',
    'User',
    'NoPass',
    'usernopass@email.com',
    '',
    'f'
),
(
    'userCanLogin',
    'User',
    'CanLogin',
    'usercanlog@email.com',
    '$argon2i$v=19$m=65536,t=3,p=1$6JGByse/9Ous9DCnkgfFnA$lrixZa334c0rLb0k8SWK67q6TtSWoYjwXje67aKK0cU',
    'f'
)
;
