-- Add down migration script here
-- Remove the seeded mock users. Scoped by the mock email domain so we
-- don't touch real rows that happen to share a username.
DELETE FROM users
WHERE email LIKE '%@example.com'
  AND provider IN ('google', 'discord');