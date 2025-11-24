# process_sub

process_sub is a tool that can remove the unnecessary chars in subtitle file, support ass and srt format.

## Install
```
cargo install --path .
```

## Usage
```
~/.cargo/bin/process_sub xx.srt > output.srt
```

## input -> output
### ass file
```
[Events]
Dialogue: 0,0:00:50.94,0:00:55.35,Default,,0,0,0,,嗯 像极了\N{\rjp}うむ よく似ておる
```
process to
```
[Events]
Dialogue: 0,0:00:50.94,0:00:55.35,Default,,0,0,0,,{\rjp}うむ よく似ておる
```
### srt file
```
2
00:02:40,677 --> 00:02:42,162
打扰了
{\fs12\i1}(ゆき子)ごめん下さい
```
process to
```
2
00:02:40,677 --> 00:02:42,162
{\fs12\i1}(ゆき子)ごめん下さい
```
