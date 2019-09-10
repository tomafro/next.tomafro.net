---
title: "htmlq"
homepage: "https://github.com/tomafro/htmlq"
---
A very quick rust command line tool to query html, in a similar way to `jq`.
```
❯ htmlq "li:nth-child(1) i.fad" ./path/to/file.html
<i class="fad fa-tv-retro"></i>
```
