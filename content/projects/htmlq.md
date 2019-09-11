---
title: "htmlq"
homepage: "https://github.com/tomafro/htmlq"
---
A very quick rust command line tool to query html, in a similar way to `jq`.
```
❯ cat ./path/to/file.html | htmlq "li:nth-child(1) i.fad"
<i class="fad fa-tv-retro"></i>
```
