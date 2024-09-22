DROP TABLE IF EXISTS course;

CREATE TABLE course 
( 
    ID serial PRIMARY KEY, 
    teacher_id INT NOT NULL, 
    NAME VARCHAR ( 140 ) NOT NULL, 
    TIME TIMESTAMP DEFAULT now( ) 
);

INSERT INTO course ( ID, teacher_id, NAME, TIME )
VALUES
	( 1, 1, 'First Course', '2023-05-18 21:30:00' );

INSERT INTO course ( ID, teacher_id, NAME, TIME )
VALUES
	( 2, 1, 'Second Course', '2023-05-28 08:45:00' );
