# html/semantics/document-metadata/the-link-element/link-type-attribute.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-type-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=match href=link-type-attribute-ref.html>
<link rel="stylesheet" type="application/javascript" href="data:text/css,div { background-color: red !important; }">
<link rel="stylesheet" type="ABCtext/css" href="data:text/css,div { background-color: red !important; }">
<link rel="stylesheet" type="text/cssDEF" href="data:text/css,div { background-color: red !important; }">
<link rel="stylesheet" type="text/invalid" href="data:text/css,div { background-color: red !important; }">
<link rel="stylesheet" type="invalid" href="data:text/css,div { background-color: red !important; }">
<p>You should see a green rectangle below</p>
<div style="width:100px;height:100px;background-color:green"></div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: red !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 183,
        "byte_start": 67,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: red !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 289,
        "byte_start": 184,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: red !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 395,
        "byte_start": 290,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: red !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 502,
        "byte_start": 396,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.link.type.invalid",
      "message": "Bad value “invalid” for attribute “type” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 604,
        "byte_start": 503,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/css,div { background-color: red !important; }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 604,
        "byte_start": 503,
        "col": 1,
        "line": 7
      }
    },
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-type-attribute.html"
}
```
