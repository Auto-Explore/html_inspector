# html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/body-bgcolor-attribute-change.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/body-bgcolor-attribute-change.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/obsolete.html#attr-body-bgcolor">
<link rel="match" href="body-bgcolor-attribute-change-ref.html">
<body bgcolor="yellow">
  Passes if the background is green.
</body>
<script>
  document.body.offsetTop;
  document.body.setAttribute("bgcolor", "green");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 254,
        "byte_start": 246,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 332,
        "byte_start": 254,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 341,
        "byte_start": 332,
        "col": 1,
        "line": 10
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
  "source_name": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/body-bgcolor-attribute-change.html"
}
```
