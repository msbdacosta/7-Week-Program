CREATE DATABASE week1;
\c week1;

CREATE TABLE people (
 id SERIAL PRIMARY KEY,
 name TEXT,
 age INT,
 country TEXT
);

INSERT INTO people (name, age, country) VALUES
('Matthew', 26, 'Brazil'),
('Aurora', 3, 'Norway'),
('Raquel', 26, 'Brazil'),
('Tsunoda', 26, 'Japan'),
('Felipe', 26, 'Canada'),
('Michael', 26, 'Hawaii');
