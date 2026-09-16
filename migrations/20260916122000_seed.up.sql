-- Seed 10 mock users. id and created_at are left to their DEFAULTs.
INSERT INTO users (provider, email, username, display_name, avatar_url) VALUES
                                                                            ('google',  'alice@example.com',   'alice',   'Alice',   'https://i.pravatar.cc/150?u=alice'),
                                                                            ('discord', 'bob@example.com',     'bob',     'Bob',     'https://i.pravatar.cc/150?u=bob'),
                                                                            ('google',  'carol@example.com',   'carol',   'Carol',   'https://i.pravatar.cc/150?u=carol'),
                                                                            ('discord', 'dave@example.com',    'dave',    'Dave',    'https://i.pravatar.cc/150?u=dave'),
                                                                            ('google',  'erin@example.com',    'erin',    'Erin',    'https://i.pravatar.cc/150?u=erin'),
                                                                            ('discord', 'frank@example.com',   'frank',   'Frank',   'https://i.pravatar.cc/150?u=frank'),
                                                                            ('google',  'grace@example.com',   'grace',   'Grace',   'https://i.pravatar.cc/150?u=grace'),
                                                                            ('discord', 'heidi@example.com',   'heidi',   'Heidi',   'https://i.pravatar.cc/150?u=heidi'),
                                                                            ('google',  'ivan@example.com',    'ivan',    'Ivan',    'https://i.pravatar.cc/150?u=ivan'),
                                                                            ('discord', 'judy@example.com',    'judy',    'Judy',    'https://i.pravatar.cc/150?u=judy');