SELECT country, COUNT(*) AS count
FROM people
GROUP BY country;