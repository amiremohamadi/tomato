<h1 align="center">
  <img src="docs/logo.png" alt="tomato" width="200">
  <br>tomato<br>
</h1>

**Tomato** is a rule-based tunnel in Rust, inspired by [Clash](https://github.com/Dreamacro/clash).

The idea behind tomato is to have a proxy client which follows two main goals:
- Performance
- Make rules as flexible as possible
(thanks to cloudflare's [wirefilter](https://github.com/cloudflare/wirefilter))

## Usage
```sh
$ ./tomato -c config.yaml
```

## Sample Config
```yaml
port: 1999
bind: '*'

log-level: 'debug'

proxies:
  - name: 'proxy'
    type: 'http'
    host: 'localhost'
    port: 8080

  - name: 'direct'
    type: 'direct'

rules:
  - rule: '"secret" in request.headers.keys'
    proxy: direct

  - rule: 'ip.geoip.country == "IR"'
    proxy: direct

  - rule: 'match'
    proxy: proxy
```
