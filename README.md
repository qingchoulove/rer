## rEr
---
quick rename resource filename.

```shell
Usage: rer [OPTIONS] --path <PATH> --regex <REGEX> --name <NAME> --year <YEAR>

Options:
      --path <PATH>
      --regex <REGEX>
      --name <NAME>
      --year <YEAR>
      --source <SOURCE>    [possible values: web-dl, hdtv, dvd]
      --clarity <CLARITY>  [possible values: 720p, 1080p, 2k, 4k]
      --encode <ENCODE>    [possible values: h264, h265, hevc]
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```shell
rer --path "/share/Downloads" --regex "Thirteen_Years_of_Dust_2023_S(?P<season>\d{2})E(?P<ep>\d{2})_1080p" --name Thirteen.Years.of.Dust --year 2023
```