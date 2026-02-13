# html/dom/elements/global-attributes/dir-shadow-37-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-shadow-37-ref.html",
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
<title>[dir] and shadow slots: dir=auto and text in a div in the shadow tree, text in the light tree</title>
<link rel="author" title="Eric Meyer" href="mailto:emeyer@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/C#the-dir-attribute">
<link rel="help" href="https://github.com/whatwg/html/issues/3699">
<link rel="help" href="https://github.com/whatwg/html/pull/9796">
<style type="text/css">
body {width: 600px;}
div {border: 1px solid gray; margin: 1em; padding: 0.25em;}
span {border: 1px solid silver;}
</style>
</head>
<body>

<p>`dir=auto` and text in a div in the shadow tree, text in the light tree</p>
<div id="host" dir="rtl">اختبر.123 456.</div>
<p id="result">The HTML direction / computed CSS `direction` value for the light tree text is: rtl / rtl (on the div) and ltr / ltr (on the host).</p>

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
        "byte_end": 475,
        "byte_start": 452,
        "col": 1,
        "line": 10
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
  "source_name": "html/dom/elements/global-attributes/dir-shadow-37-ref.html"
}
```
