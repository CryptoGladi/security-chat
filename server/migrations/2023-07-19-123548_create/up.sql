CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  nickname VARCHAR(25) UNIQUE NOT NULL,
  authkey VARCHAR(40) NOT NULL
);

CREATE TABLE chat (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(40) NOT NULL
);

CREATE TABLE chat_messages (
  id BIGSERIAL PRIMARY KEY,
  chat_id BIGSERIAL REFERENCES chat(id) ON DELETE CASCADE,
  sender_id BIGSERIAL REFERENCES users(id),
  message JSON NOT NULL
);

CREATE TABLE order_add_keys ( -- only for chat
  id BIGSERIAL PRIMARY KEY,
  user_to_id BIGSERIAL REFERENCES users(id) ON DELETE CASCADE,
  user_from_id BIGSERIAL REFERENCES users(id) ON DELETE CASCADE,
  user_to_public_key BYTEA NOT NULL,
  user_from_public_key BYTEA
);