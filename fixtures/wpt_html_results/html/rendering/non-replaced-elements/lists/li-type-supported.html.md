# html/rendering/non-replaced-elements/lists/li-type-supported.html

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/li-type-supported.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>li@type: supported types</title>
<link rel=match href=li-type-supported-ref.html>
<li type=1>first item</li>
<li type=a>second item</li>
<li type=A>third item</li>
<li type=i>fourth item</li>
<li type=I>fifth item</li>
<li type=disc>sixth item</li>
<li type=circle>seventh item</li>
<li type=square>eighth item</li>
<li type=none>ninth item</li>
<ol>
  <li type=1>first ordered item</li>
  <li type=a>second ordered item</li>
  <li type=A>third ordered item</li>
  <li type=i>fourth ordered item</li>
  <li type=I>fifth ordered item</li>
  <li type=disc>sixth ordered item</li>
  <li type=circle>seventh ordered item</li>
  <li type=square>eighth ordered item</li>
  <li type=none>ninth ordered item</li>
</ol>
<ul>
  <li type=1>first unordered item</li>
  <li type=a>second unordered item</li>
  <li type=A>third unordered item</li>
  <li type=i>fourth unordered item</li>
  <li type=I>fifth unordered item</li>
  <li type=disc>sixth unordered item</li>
  <li type=circle>seventh unordered item</li>
  <li type=square>eighth unordered item</li>
  <li type=none>ninth unordered item</li>
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
        "byte_end": 137,
        "byte_start": 126,
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
        "byte_end": 164,
        "byte_start": 153,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 192,
        "byte_start": 181,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 219,
        "byte_start": 208,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 247,
        "byte_start": 236,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 277,
        "byte_start": 263,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 309,
        "byte_start": 293,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 343,
        "byte_start": 327,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 374,
        "byte_start": 360,
        "col": 1,
        "line": 13
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
  "source_name": "html/rendering/non-replaced-elements/lists/li-type-supported.html"
}
```
