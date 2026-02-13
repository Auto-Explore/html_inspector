# html/rendering/non-replaced-elements/lists/li-type-unsupported-lower-alpha.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/li-type-unsupported-lower-alpha.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>li@type: unsupported type: lower-alpha</title>
<link rel=match href=li-type-unsupported-ref.html>
<li type=lower-alpha>first item</li>
<li type=LOWER-ALPHA>second item</li>
<ol>
  <li type=lower-alpha>first ordered item</li>
  <li type=LOWER-ALPHA>second ordered item</li>
</ol>
<ul>
  <li type=lower-alpha>first unordered item</li>
  <li type=LOWER-ALPHA>second unordered item</li>
</ul>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 163,
        "byte_start": 142,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 200,
        "byte_start": 179,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/non-replaced-elements/lists/li-type-unsupported-lower-alpha.html"
}
```
