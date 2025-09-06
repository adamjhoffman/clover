# clover

clover is a cli tool to keep track of weekly tasks and classes throughout the semester written in rust. 

## Install

`cargo install --git https://github.com/adamjhoffman/clover`

## Usage

- Add a class using `clover addclass` followed by the class name.
- Add a task using `clover addtask` followed by the task name.
- Configure the length of your semester in weeks using `clover settime` followed by a number.
- Complete a task for a class in a specific week using `clover complete` and supplying the `--class`, `--task` and `--week`.
- Revert an accidentally completed taks using `clover revert`.
- Display the current overview using `clover show`.

### Example

```
% clover addclass math
Added class math
% clover addclass physics
Added class physics
% clover addtask homework
Added task homework
% clover addtask notes
Added task notes
% clover settime 14
Set week count to 14
% clover complete --task homework --class math --week 0
Completed homework for math in week 0
% clover show
------------------------------------------------
         | math             | physics          |
         ---------------------------------------
         | homework | notes | homework | notes |
------------------------------------------------
 week 00 | ████████ |       |          |       |
------------------------------------------------
 week 01 |          |       |          |       |
------------------------------------------------
 week 02 |          |       |          |       |
------------------------------------------------
 week 03 |          |       |          |       |
------------------------------------------------
 week 04 |          |       |          |       |
------------------------------------------------
 week 05 |          |       |          |       |
------------------------------------------------
 week 06 |          |       |          |       |
------------------------------------------------
 week 07 |          |       |          |       |
------------------------------------------------
 week 08 |          |       |          |       |
------------------------------------------------
 week 09 |          |       |          |       |
------------------------------------------------
 week 10 |          |       |          |       |
------------------------------------------------
 week 11 |          |       |          |       |
------------------------------------------------
 week 12 |          |       |          |       |
------------------------------------------------
 week 13 |          |       |          |       |
------------------------------------------------
```

## Config

By default, the config is store in `~/.clover`. A custom config file can by supplied by using `--config`. The data is serialized to json using serde.
