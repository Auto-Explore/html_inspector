# html/semantics/document-metadata/the-style-element/html_style_in_comment.xhtml

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/html_style_in_comment.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<link rel="match" href="html_style_in_comment-ref.html"/>
<style type="text/css">
h4 {color: green}
<!--
h4 {color: red}
-->
</style>
</head>
<body>
<p> This page tests that Style written inside HTML comment is not applied</p>
This test passes if the text below is <b>Green. NOT Red.</b>
<h4>
This is some text.
</h4>
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
        "byte_end": 132,
        "byte_start": 109,
        "col": 1,
        "line": 4
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
  "source_name": "html/semantics/document-metadata/the-style-element/html_style_in_comment.xhtml"
}
```
