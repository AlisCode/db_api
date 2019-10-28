-- Your SQL goes here
CREATE TABLE hero (
    id INTEGER PRIMARY KEY NOT NULL,
    year INTEGER NOT NULL,
    name VARCHAR(30) NOT NULL,
    power VARCHAR(30) NOT NULL 
);

INSERT INTO hero VALUES(1, 1997, 'Olivier Pinon', 'SuperDev');
INSERT INTO hero VALUES(2, 1938, 'Superman', 'Strength and speed');