CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  nickname VARCHAR(25) UNIQUE NOT NULL,
  authkey VARCHAR(40) NOT NULL
);

CREATE TABLE conversation (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(40) NOT NULL
);

CREATE TABLE conversation_messages (
  id BIGSERIAL PRIMARY KEY,
  conversation_id BIGSERIAL REFERENCES conversation(id) ON DELETE CASCADE,
  sender_nickname VARCHAR(40) REFERENCES users(nickname),
  message JSON NOT NULL
);

--CREATE TABLE order_add_keys {
--  id BIGSERIAL PRIMARY KEY,
--  user_to VARCHAR(40) REFERENCES users(nickname),
--  user_from VARCHAR(40) REFERENCES users(nickname)
--  
--}