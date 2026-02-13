# html/rendering/non-replaced-elements/lists/li-type-supported-xhtml.xhtml

Counts:
- errors: 0
- warnings: 10
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/li-type-supported-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>li@type: supported types</title>
<link rel="match" href="li-type-supported-ref.html"/>
</head>
<body>
<li type="1">first item</li>
<li type="a">second item</li>
<li type="A">third item</li>
<li type="i">fourth item</li>
<li type="I">fifth item</li>
<li type="disc">sixth item</li>
<li type="circle">seventh item</li>
<li type="square">eighth item</li>
<li type="none">ninth item</li>
<ol>
  <li type="1">first ordered item</li>
  <li type="a">second ordered item</li>
  <li type="A">third ordered item</li>
  <li type="i">fourth ordered item</li>
  <li type="I">fifth ordered item</li>
  <li type="disc">sixth ordered item</li>
  <li type="circle">seventh ordered item</li>
  <li type="square">eighth ordered item</li>
  <li type="none">ninth ordered item</li>
</ol>
<ul>
  <li type="1">first unordered item</li>
  <li type="a">second unordered item</li>
  <li type="A">third unordered item</li>
  <li type="i">fourth unordered item</li>
  <li type="I">fifth unordered item</li>
  <li type="disc">sixth unordered item</li>
  <li type="circle">seventh unordered item</li>
  <li type="square">eighth unordered item</li>
  <li type="none">ninth unordered item</li>
</ul>
</body>
</html>
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
        "byte_end": 212,
        "byte_start": 199,
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
        "byte_end": 241,
        "byte_start": 228,
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
        "byte_end": 271,
        "byte_start": 258,
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
        "byte_end": 300,
        "byte_start": 287,
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
        "byte_end": 330,
        "byte_start": 317,
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
        "byte_end": 362,
        "byte_start": 346,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 396,
        "byte_start": 378,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 432,
        "byte_start": 414,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 465,
        "byte_start": 449,
        "col": 1,
        "line": 16
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
  "source_name": "html/rendering/non-replaced-elements/lists/li-type-supported-xhtml.xhtml"
}
```
