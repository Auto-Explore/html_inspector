# html/semantics/embedded-content/the-iframe-element/same_origin_parentage.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/same_origin_parentage.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Check the frame heriarchy</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="iframe_harness.js"></script>
<body>
  <iframe src="same_origin_child.html"></iframe>
</body>
<script>
    get_test_results('17381dae-9c3e-4661-9f2b-28eb07a5f2fc');
    get_test_results('08782f28-e313-47ae-8cd7-419f3e194b0a');
    get_test_results('66de8d44-7da7-47c7-9a52-41cba4f22bfe');
    send_test_results({
        "id": '17381dae-9c3e-4661-9f2b-28eb07a5f2fc',
        "parent": window.parent === window,
        "top": window.top === window,
    });
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
        "byte_end": 300,
        "byte_start": 292,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 655,
        "byte_start": 300,
        "col": 9,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 664,
        "byte_start": 655,
        "col": 1,
        "line": 19
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/same_origin_parentage.html"
}
```
