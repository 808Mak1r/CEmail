# CEmail

<p align="center">
  红队作战中钓鱼环节针对目标邮箱发送的可能性，可以用来枚举目标存在哪些邮箱。
</p>

<p align="center">
  <a href="#安装">安装</a> •
  <a href="#使用">使用</a> •
  <a href="#免责声明">免责声明</a>
  <br>
  [中文]
  [<a href="./README-en.md">English</a>]
</p>

## 安装

### 二进制

可以通过 `Github action`编译的文件下载直接使用。

### 编译

1. 参考[ Rust官方教程 ](https://www.rust-lang.org/tools/install)进行安装环境。
2. 编译

```bash
$ cargo build --locked --release
```

编译文件存放在 `target/release` 目录下面

## 使用

```bash
$ ./cemail -h                                                                                         

cemail 0.1.0
808Mak1r <808Mak1r@gmail.com>
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

状态：
- `Safe`: 表示可送达
- `Risky`: 表示存在风险不一定
- `Invalid`: 表示不存在
- `Unknown`


- `-e` 检测单个目标邮箱，默认保存到`output.json`

```bash
$ ./cemail -e 808Mak1r@gmail.com
2022-10-19 19:34:15.948  INFO 开始读取邮箱
2022-10-19 19:34:15.949  INFO 邮箱读取完毕
2022-10-19 19:34:18.478  INFO 邮箱: 808Mak1r@gmail.com 状态: Safe
2022-10-19 19:34:18.481  INFO 输出到文件: output.json
```

详细信息默认输出到 `json` 中查看，格式如下：
```json
{
  "safe": [
    "808Mak1r@gmail.com"
  ],
  "risky": [],
  "invalid": [],
  "unknown": [],
  "ce_output": [
    {
      "input": "808Mak1r@gmail.com",
      "is_reachable": "safe",
      "misc": {
        "is_disposable": false,
        "is_role_account": false
      },
      "mx": {
        "accepts_mail": true,
        "records": [
          "alt2.gmail-smtp-in.l.google.com.",
          "alt4.gmail-smtp-in.l.google.com.",
          "gmail-smtp-in.l.google.com.",
          "alt3.gmail-smtp-in.l.google.com.",
          "alt1.gmail-smtp-in.l.google.com."
        ]
      },
      "smtp": {
        "can_connect_smtp": true,
        "has_full_inbox": false,
        "is_catch_all": false,
        "is_deliverable": true,
        "is_disabled": false
      },
      "syntax": {
        "address": "808Mak1r@gmail.com",
        "domain": "gmail.com",
        "is_valid_syntax": true,
        "username": "808Mak1r"
      }
    }
  ]
}
```


- `-f` 批量检测目标邮箱，并将详细结果保存到 `test_out.json`

可以通过`-o`指定输出文件名，不指定会默认保存为`output.json`，如果已经存在会按照顺序递归

```bash
$ ./cemail -f test.txt -o test_out.json
```

## 免责声明

本工具仅面向**合法授权**的企业安全建设行为，如您需要测试本工具的可用性，请自行搭建靶机环境。

为避免被恶意使用，本项目所有收录的poc均为漏洞的理论判断，不存在漏洞利用过程，不会对目标发起真实攻击和漏洞利用。

在使用本工具进行检测时，您应确保该行为符合当地的法律法规，并且已经取得了足够的授权。**请勿对非授权目标进行扫描。**

如您在使用本工具的过程中存在任何非法行为，您需自行承担相应后果，我们将不承担任何法律及连带责任。

在安装并使用本工具前，请您**务必审慎阅读、充分理解各条款内容**，限制、免责条款或者其他涉及您重大权益的条款可能会以加粗、加下划线等形式提示您重点注意。
除非您已充分阅读、完全理解并接受本协议所有条款，否则，请您不要安装并使用本工具。您的使用行为或者您以其他任何明示或者默示方式表示接受本协议的，即视为您已阅读并同意本协议的约束。
