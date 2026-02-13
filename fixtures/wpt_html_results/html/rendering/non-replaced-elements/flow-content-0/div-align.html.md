# html/rendering/non-replaced-elements/flow-content-0/div-align.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/div-align.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset=utf-8>
<link rel="match" href="div-align-ref.html">
<style>
.test { width: 50px; background-color: yellow; }
.rtl { direction: rtl; }
.ltr { direction: ltr; }
[align=left] .margin { margin-left: 1em }
[align=right] .margin { margin-right: 1em }
</style>
</head>
<body>
<!-- Centered tests -->
<div align=center>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div align=center>
<div class=test align=left>t א</div>
<div class=test align=right>t א</div>
</div>

<div align=left>
<div align=center>
<div class=test>t א</div>
</div>
</div>

<!-- Left-aligned tests -->
<div align=left>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div align=left class=rtl>
<div class=test>t א</div>
<div class="test ltr">t א</div>
<div class="test margin">t א</div>
</div>

<div align=left>
<div class=test align=center>t א</div>
<div class=test align=right>t א</div>
</div>

<!-- Right-aligned tests -->
<div align=right>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div align=right class=rtl>
<div class=test>t א</div>
<div class="test ltr">t א</div>
<div class="test margin">t א</div>
</div>

<div align=right>
<div class=test align=left>t א</div>
<div class=test align=center>t א</div>
</div>

</body>
</html>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/div-align.html"
}
```
