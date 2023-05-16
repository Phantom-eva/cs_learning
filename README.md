# Computer Science Study Plan

## 1. Programming Languages

### 1.1. Shell

#### Shell Command

`date`: print the current date and time

`echo argument`: print out the arguments

`which program`: which file is executed for a given program

`pwd`: the current working directory

`cd`: change the working directory

`.`: the current directory

`..`: its parent directory

`ls`: print the contents of the current directory

`ls -l`: show what permissions the owner group, the owning group, and everyone else have

`mv`: rename/move a file

`cp`: copy a file

`mkdir`: make a new directory

`curl`: transfer data to or from a server

`grep`: find or search a regular expression or a string in a text file

`echo 3 | sudo tee brightness`: open the file and write 3 into it

#### Shell Scripting

`$0`: Name of the script

`$1~$9`: Arguments to the script

`$@`: All the arguments

`$#`: Number of arguments

`$?`: Return code of the previous command

`$$`: Process identification number for the current script

`!!`: Entire last command, including arguments

`$_`: Last argument from the last command

### 1.2. SQL

```sql
SELECT * FROM R WHERE a_id = 'a2';
```

```sql
SELECT a_id, b_id FROM R WHERE a_id = 'a2';
```

```sql
(SELECT * FROM R) UNION ALL (SELECT * FROM S);
```

```sql
(SELECT * FROM R) INTERSECT (SELECT * FROM S);
```

```sql
(SELECT * FROM R) EXCEPT (SELECT * FROM S);
```

```sql
(SELECT * FROM R) CROSS JOIN (SELCT * FROM S);
```

```sql
SELECT * FROM R JOIN S USING (ATTRIBUTE1, ATTRIBUTE2...);
```

```sql
CREATE TABLE student (
	sid INT PRIMARY KEY,
	name VARCHAR(16),
	login VARCHAR(32) UNIQUE,
	age SMALLINT,
	gpa FLOAT
);
CREATE TABLE course (
	cid VARCHAR(32) PRIMARY KEY,
	name VARCHAR(32) NOT NULL
);
CREATE TABLE enrolled (
	sid INT REFERENCES student (sid),
	cid VARCHAR(32) REFERENCES course (cid),
	grade CHAR(1)
);
```

```sql
SELECT AVG(gpa), COUNT(sid) FROM student WHERE login LIKE '%@cs';
```

```sql
SELECT AVG(s.gpa), e.cid
FROM enrolled AS e, student AS s
WHERE e.sid = s.sid
GROUP BY e.cid
HAVING AVG(s.gpa) > 3.9;
```

```sql
SELECT DISTINCT cid INTO CourseIds FROM enrolled
```

### 1.3. C++

### 1.4. Python

### 1.5. Java

### 1.6. Rust

## 7. Data Structures and Algorithms

## 8. Operating System

## 9. Computer Architecture

## 10. Distributed System

## 11. Computer Security

## 12. Computer Network

## 13. Database System

## 14. Compilers

