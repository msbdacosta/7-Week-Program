SELECT name, age,
  CASE
    WHEN age >= 20 THEN 'Adult'
    WHEN age >= 13 THEN 'Teenager'
    ELSE 'Child'
  END AS age_group
FROM people;