# Computer Science Study Plan

## 1. Programming Languages

### 1.1. Shell (MIT-Missing-Semester)

#### Shell Command

````shell
date # print the current date and time
````

````shell
echo argument # print out the arguments
````

```shell
which program # which file is executed for a given program
```

```shell
pwd # the current working directory
```

```shell
cd # change the working directory
```

```shell
. # the current directory
```

```shell
.. # its parent directory
```

```shell
ls # print the contents of the current directory
```

```shell
ls -l # show what permissions the owner group, the owning group, and everyone else have
```

```shell
mv # rename/move a file
```

```shell
cp # copy a file
```

```shell
mkdir # create a new directory
```

```shell
touch # create a new file
```

```shell
man program # show the manual page of a program
```

```shell
cat file # print the contents of a file
```

```shell
curl # transfer data to or from a server
```

```shell
grep # find or search a regular expression or a string in a text file
```

```shell
echo 3 | sudo tee brightness # open the file and write 3 into it
```

```shell
chmod 777 program # change permissions of a program
```

```shell
find . -type f -name '*.py' # find all python files
```

```shell
rg -t py 'import requests' # find all python files where I used the requests library
```

```shell
tar -czvf test.zip # compress the file
tar -tzvf test.zip # show the contents of the zip
tar -xzvf test.zip # unzip the file
```

```shell
find . -type f -print0 | xargs -0 ls -l -t | head -1
```

#### Shell Scripting

```shell
$0 # Name of the script
```

```shell
$1~$9 # Arguments to the script
```

```shell
$@ # All the arguments
```

```shell
$# # Number of arguments
```

```shell
$? # Return code of the previous command
```

```shell
$$ # Process identification number for the current script
```

```shell
!! # Entire last command, including arguments
```

```shell
$_ # Last argument from the last command
```

### 1.2. SQL

```sql
SELECT * FROM R WHERE a_id = 'a2'; # select
```

```sql
SELECT a_id, b_id FROM R WHERE a_id = 'a2'; # projection
```

```sql
(SELECT * FROM R) UNION ALL (SELECT * FROM S); # union
```

```sql
(SELECT * FROM R) INTERSECT (SELECT * FROM S); # intersection
```

```sql
(SELECT * FROM R) EXCEPT (SELECT * FROM S); # difference
```

```sql
(SELECT * FROM R) CROSS JOIN (SELCT * FROM S); # product
```

```sql
SELECT * FROM R JOIN S USING (ATTRIBUTE1, ATTRIBUTE2...); # join
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
); # create table
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

### 1.3. C++ (Stanford CS106L)

### 1.4. Python (UCB CS61A)

### 1.5. Java

### 1.6. Rust (Stanford CS110L)

## 7. Data Structures and Algorithms

## 8. Operating System

## 9. Computer Architecture

## 10. Distributed System

## 11. Computer Security

## 12. Computer Network

## 13. Database System (CMU 15-445/645)



## 14. Compilers

