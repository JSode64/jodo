# Jodo
Jodo is a simple command-line todo list application.

## Usage
Jodo uses a file stored in your %APPDATA% directory to store jobs/tasks to be completed.

To view the current list of tasks, run the program with no arguments.
To add a task, use "-a".
To remove a task, use "-r".

## Examples
Adding tasks:
```
jodo -a task1 "task two" "task 3"
jodo
0: task1
1: task two
2: task 3
```

Removing tasks:
```
jodo -r 0 2
jodo
0: task two
```

Adding AND removing tasks
```
jodo -a "new task" hello -r 0
jodo
0: new task
1: hello
```
