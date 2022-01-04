# `what-was-that`

A simple tool to remember little things.

## Motivation

> Hey, I can't remember the commmand to convert this file to Markdown?
> What was the port on which my local IPFS server was running?

Did something like this ever happen to you? Well, I'm sure it did.
This forgetfulness results in lots of time wasted searching on the Internet for
the same thing over and over again. This is why I created `what-was-that`. I can put in the command and its description (i.e what it does) and it'll save all of those in a file. I can then search for the command using the description and it will print the command. Do you find this useful? Go on reading to find how to install it.

## Installation

Self-install:

```
git clone https://github.com/obnoxiousnerd/what-was-that.git

cd what-was-that

cargo install
```

**OR**

Download the binary from GitHub Releases, and put it somewhere in your PATH.

## Usage

Check out the help section:

```
what-was-that --help
```

### Examples

Remember something:

```
what-was-that remember "ls -l" "List the contents of the current directory"
```

Find something by describing it:

```
what-was-that find "list contents of current directory"
```

Forget something forever (unless you add it back again):

```
# You have to type the exact command in the arguments to delete it
what-was-that forget "ls -l"
```
