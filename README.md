# Overview

This is a CLI application that prints out a Life Calendar in Weeks in the terminal.

Execution example:
```
cargo run -- --year 2012 --month 4 --day 2
```
Output:
```
Life Calendar in weeks.
---------------------------------------------------------------------------------------------------------------------------------------------------------------
  | 01      04| 05      08| 09      12| 13      16| 17      20| 21      24| 25      28| 29      32| 33      36| 37      40| 41      44| 45      48| 49      52|
---------------------------------------------------------------------------------------------------------------------------------------------------------------
01| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
02| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
03| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
04| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
05| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
06| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
07| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
08| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
09| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
10| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|

11| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
12| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
13| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x| x  x  x  x|
14| x  x  x  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
15| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
16| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
17| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
18| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
19| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
20| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .| .  .  .  .|
---------------------------------------------------------------------------------------------------------------------------------------------------------------
  | 01      04| 05      08| 09      12| 13      16| 17      20| 21      24| 25      28| 29      32| 33      36| 37      40| 41      44| 45      48| 49      52|
---------------------------------------------------------------------------------------------------------------------------------------------------------------
```
