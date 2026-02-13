# html/semantics/links/linktypes/link-type-stylesheet/process-stylesheet-linked-resource-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/linktypes/link-type-stylesheet/process-stylesheet-linked-resource-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#link-type-stylesheet:process-the-linked-resource">
<link rel="help" href="https://html.spec.whatwg.org/#content-type">
<link rel="help" href="https://mimesniff.spec.whatwg.org/#mime-type-representation">
<link rel="match" href="process-stylesheet-linked-resource-ascii-case-insensitive-ref.html">
<meta name="assert" content="link@type values for stylesheet resources are ASCII case-insensitive">
<link rel="stylesheet" href="process-stylesheet-linked-resource-ascii-case-insensitive.css">
<link rel="stylesheet" href="process-stylesheet-linked-resource-ascii-case-insensitive-lower.css" type="text/css">
<link rel="stylesheet" href="process-stylesheet-linked-resource-ascii-case-insensitive-mixed.css" type="TeXt/CsS">
<link rel="stylesheet" href="process-stylesheet-linked-resource-ascii-case-insensitive-other.css" type="text/cſs">
<p id="z-lower">text/css treated as CSS?
<p id="z-mixed">TeXt/CsS treated as CSS?
<p id="z-other">text/cſs ignored?
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.type.invalid",
      "message": "Bad value “text/cſs” for attribute “type” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 927,
        "byte_start": 812,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/links/linktypes/link-type-stylesheet/process-stylesheet-linked-resource-ascii-case-insensitive.html"
}
```
