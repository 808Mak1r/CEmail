# CEmail

<p align="center">
  The possibility of the fishing link for the fishing link in the red team can be used to enumerate what mailboxes exist in the target.
</p>

<p align="center">
  <a href="#Install">Install</a> •
  <a href="#Use">Use</a> •
  <a href="#Exempt statement">Exempt statement</a>
  <br>
  [<a href="./README.md">中文</a>]
  [English]
</p>

## Install

### Binary

You can download and use the compiled files directly by `Github action`

### Compile

1. Reference[ Rust official tutorial ](https://www.rust-lang.org/tools/install)Installation environment.
2. Compile

```bash
$ cargo build --locked --release
```

Compiled files stored in `target/release`under the directory.

## Use

```bash
$ ./cemail -h                                                                                                                                            main ✖ ✱ ◼
cemail 0.1.0
A simple CLI to check if an email exists

USAGE:
    cemail [OPTIONS]

OPTIONS:
    -e, --email <EMAIL>      The email to check
    -f, --file <FILE>        The file containing the list of emails to check
    -h, --help               Print help information
    -o, --output <OUTPUT>    The output file [default: output.json]
    -V, --version            Print version information
```

State:
- `Safe`: Express can be delivered
- `Risky`: It means that the risk is not necessarily
- `Invalid`: Indicate that there is no existence

Detect a single target mailbox

```bash
$ ./cemail -e 808Mak1r@gmail.com
2022-10-19 19:34:15.948  INFO 开始读取邮箱
2022-10-19 19:34:15.949  INFO 邮箱读取完毕
2022-10-19 19:34:18.478  INFO 邮箱: 808Mak1r@gmail.com 状态: Safe
2022-10-19 19:34:18.481  INFO 输出到文件: output.json
```

Details default output to `json` to view

Check the target mailbox in batches, and save the detailed results to `test_out.json`

```bash
$ ./cemail -f test.txt -o test_out.json
```

## Exempt statement

This tool is only facing the legal construction behavior of legal authorization. If you need to test the availability of this tool, please build the target environment by yourself.

In order to avoid malicious use, all the POCs included in this project are theoretical judgments of vulnerabilities. There is no vulnerability utilization process, and real attacks and vulnerabilities will not be launched on the target.

When using this tool for testing, you should ensure that the behavior is in line with local laws and regulations and has obtained sufficient authorization.Do not scan the non -authorized target.

If you have any illegal acts in the process of using this tool, you need to bear the corresponding consequences, and we will not bear any law and joint responsibilities.

Before installing and using this tool, please be careful to read and fully understand the content of each terms,restrictions, exemption clauses, or other terms involving your major rights and interests may prompt you to pay attention to your focus on boldness, plus lines and other forms. Essence

Unless you have fully read, understand and accept all the terms of this agreement, please do not install and use this tool. If your use behavior or any other explicit or implied way to accept this agreement, that is, it is deemed to have read and agree with the constraints of this agreement.
