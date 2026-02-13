# html/dom/elements/global-attributes/dir-shadow-16.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-shadow-16.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8"/>
<title>[dir] and shadow slots: dir=rtl on the shadow host, paragraph in the shadow tree</title>
<link rel="author" title="Eric Meyer" href="mailto:emeyer@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/C#the-dir-attribute">
<link rel="help" href="https://github.com/whatwg/html/issues/3699">
<link rel="help" href="https://github.com/whatwg/html/pull/9796">
<link rel="match" href="dir-shadow-16-ref.html">
<style type="text/css">
body {width: 600px;}
div {border: 1px solid gray; margin: 1em;}
</style>
<script src="dir-shadow-utils.js"></script>
</head>
<body>

<p>`dir=rtl` on the shadow host, paragraph in the shadow tree</p>
<div id="host" dir="rtl"></div>
<p id="result">The HTML direction / computed CSS `direction` value for the paragraph is: </p>

<script type="text/javascript">
  let root = host.attachShadow({mode:"open"});
  root.innerHTML = `<style>p {width: 50%; border: 1px dotted;}</style><p>اختبر.</p>`;
  result.innerHTML += html_direction(root.querySelector('p')) + " / " + getComputedStyle(root.querySelector('p')).direction + '.';
</script>

</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 511,
        "byte_start": 488,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 869,
        "byte_start": 838,
        "col": 1,
        "line": 23
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
  "source_name": "html/dom/elements/global-attributes/dir-shadow-16.html"
}
```
