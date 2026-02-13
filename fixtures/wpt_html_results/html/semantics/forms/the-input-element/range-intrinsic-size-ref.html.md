# html/semantics/forms/the-input-element/range-intrinsic-size-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-intrinsic-size-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<!--
     Any copyright is dedicated to the Public Domain.
     http://creativecommons.org/publicdomain/zero/1.0/
-->
<html><head>
  <meta charset="utf-8">
  <title>Reference: type=range intrinsic size</title>
  <link rel="author" title="Mats Palmgren" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1512066">
  <style>
html,body {
  color:black; background-color:white; font:16px/1 monospace;
}

.flex {
  display: inline-flex;
  width: 0;
  border: 1px solid;
  justify-items:start;
}
.flex2 {
  display: inline-flex;
  border: 1px solid;
  justify-items:start;
}
.grid {
  display: inline-grid;
  grid: auto / 0;
  border: 1px solid;
  justify-items:start;
}
.grid2 {
  display: inline-grid;
  border: 1px solid;
  justify-items:start;
}
.ib {
  display: inline-block;
  width: 0;
  border: 1px solid;
  justify-items:start;
}

input {
   width: max-content;
   min-width: 0;
}
input.min {
   min-width: min-content;
}
input.mbp0 {
  margin-left: 0;
  margin-right: 0;
  padding: 0;
  border: 0;
}
  </style>
</head>
<body>

<div class="flex"><input type="range" class="min"></div><br>
<div class="flex"><input type="range" style="width:0"></div><br>
<div class="flex"><input type="range" class="min"></div><br>
<div class="flex"><input type="range" class="min"></div><br>
<div class="flex"><input type="range" class="min"></div><br>
<br>

<div class="flex2"><input type="range"></div>
<div class="flex2" style="width:3px"><input type="range" style="width:3px" class="mbp0"></div>
<div class="flex2" style="width:30px"><input type="range" class="mbp0"></div>
<div class="flex2"><input type="range"></div>
<div class="flex2"><input type="range"></div>
<div class="flex2"><input type="range"></div>
<div class="flex2"><input type="range"></div>
<br>

<div class="grid"><input type="range" style="width:0"></div><br>
<div class="grid"><input type="range" style="width:0"></div><br>
<div class="grid" style="justify-items:start"><input type="range"></div><br>

<div class="grid2"><input type="range"></div>
<div class="grid2"><input type="range" style="min-width:0"></div>
<div class="grid2" style="width:3px"><input type="range" style="width:3px" class="mbp0"></div>
<div class="flex2" style="width:30px"><input type="range" class="mbp0"></div>
<div class="flex2" style="width:30px"><input type="range" class="mbp0"></div>
<div class="grid2" style="justify-items:start"><input type="range"></div>

<br>

<div class="ib"><input type="range"></div><br>
<div class="ib"><input type="range"></div><br>

<input type="range">
<input type="range"

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.lt_expecting_attr_name",
      "message": "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
      "severity": "Warning",
      "span": {
        "byte_end": 2568,
        "byte_start": 2540,
        "col": 1,
        "line": 94
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/range-intrinsic-size-ref.html"
}
```
