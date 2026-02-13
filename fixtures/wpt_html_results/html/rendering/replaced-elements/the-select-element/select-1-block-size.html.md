# html/rendering/replaced-elements/the-select-element/select-1-block-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-1-block-size.html",
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
  <title>Test: Combobox block-size test</title>
  <link rel="author" title="Mats Palmgren" href="mailto:mats@mozilla.com">
  <link rel="match" href="select-1-block-size-ref.html">
  <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1499578">
  <style>
html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}

select { -webkit-appearance: none; }

optgroup { font-size: 32pt; }
option { font-size: 24pt; }
.big { font-size: 48pt; }
.lh { line-height: 48pt; }

.mask { position:fixed; left:20px; right:0; top:0; bottom:0; background: black; }
  </style>
</head>
<body>

<!-- mask off differences on the right side -->
<div class="mask"></div>

<select><optgroup label="label"><option>option</option></select><br>
<select class="big"><optgroup label="label"><option>option</option></select><br>
<select class="lh"><optgroup label="label"><option>option</option></select><br>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/the-select-element/select-1-block-size.html"
}
```
