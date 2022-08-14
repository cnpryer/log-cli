[![Tests](https://github.com/cnpryer/log-cli/actions/workflows/build.yml/badge.svg)](https://github.com/cnpryer/log-cli/actions/workflows/build.yml)

# log-cli

Command line interface for log files.

This project is currently under development.

## Usage

```console
log-cli sample.log

2022-01-01 07:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 08:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 12:00:00,0 [info] module3  Message Subject: Text for a message.
2022-01-01 13:00:00,0 [info] module3  Message Subject: Text for a message.
2022-01-01 14:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 15:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 16:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 17:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 18:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 19:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-01 20:00:00,0 [debug] module5  Message Subject: Text for a message.
2022-01-01 21:00:00,0 [info] module2  Message Subject: Text for a message.
2022-01-01 22:00:00,0 [info] module2  Message Subject: Text for a message.
2022-01-01 23:00:00,0 [info] module6  Message Subject: Text for a message.
2022-01-02 00:00:00,0 [warning] module1  Message Subject: Text for a message.
2022-01-02 01:00:00,0 [info] module10  Message Subject: Text for a message.
2022-01-02 02:00:00,0 [info] module1  Message Subject: Text for a message.
2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
```

### View using keywords

```console
log-cli sample.log --keywords "[debug]" "[warning]"

2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
2022-01-01 20:00:00,0 [debug] module5  Message Subject: Text for a message.
2022-01-02 00:00:00,0 [warning] module1  Message Subject: Text for a message.
2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
```
