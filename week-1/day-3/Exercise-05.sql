SELECT name 
FROM people
WHERE age > (
	SELECT AVG(age) FROM people
);