# html/rendering/non-replaced-elements/flow-content-0/div-align-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/div-align-ref.html",
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
<style>
.test { width: 50px; background-color: yellow; }
.center { text-align: center; }
.center .test { margin: 0 auto; }
.left { text-align: left; }
.left .test { margin-right: auto; }
.right { text-align: right; }
.right .test { margin-left: auto; }
.rtl { direction: rtl; }
.ltr { direction: ltr; }
.left .margin { margin-left: 1em; }
.right .margin { margin-right: 1em; }
</style>
</head>
<body>
<!-- Centered tests -->
<div class=center>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div class=center>
<div class="test left">t א</div>
<div class="test right">t א</div>
</div>

<div class=left>
<div class=center>
<div class=test>t א</div>
</div>
</div>

<!-- Left-aligned tests -->
<div class=left>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div class="left rtl">
<div class=test>t א</div>
<div class="test ltr">t א</div>
<div class="test margin">t א</div>
</div>

<div class=left>
<div class="test center">t א</div>
<div class="test right">t א</div>
</div>

<!-- Right-aligned tests -->
<div class=right>
<div class=test>t א</div>
<div class="test rtl">t א</div>
<div class="test margin">t א</div>
</div>

<div class="right rtl">
<div class=test>t א</div>
<div class="test ltr">t א</div>
<div class="test margin">t א</div>
</div>

<div class=right>
<div class="test left">t א</div>
<div class="test center">t א</div>
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
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/div-align-ref.html"
}
```
