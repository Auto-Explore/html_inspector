# html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://github.com/servo/servo/issues/42231">
<link rel="author" title="Jo Steven Novaryo" href="mailto:steven.novaryo@gmail.com">
<link rel="match" href="caret-visibility-after-creation-in-script-ref.html">
<title>A cursor should be shown when a textarea element is created and focused immediately</title>
<body></body>
<script>
  const target = document.createElement('textarea');
  document.body.appendChild(target);
  target.focus();
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
        "byte_end": 368,
        "byte_start": 360,
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
        "byte_end": 477,
        "byte_start": 368,
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
        "byte_end": 486,
        "byte_start": 477,
        "col": 1,
        "line": 11
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
  "source_name": "html/rendering/replaced-elements/the-textarea-element/caret-visibility-after-creation-in-script.html"
}
```
