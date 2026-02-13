# html/semantics/embedded-content/the-iframe-element/cross_origin_parentage.sub.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/cross_origin_parentage.sub.html",
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
  <iframe src="http://{{domains[www1]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-iframe-element/cross_origin_child.html"></iframe>
</body>
<script>
    get_test_results('bffa23ee-b45a-4e9a-9405-87ab437d5cfa');
    get_test_results('79a52de8-4222-427e-92db-caec28e75f8e');
    get_test_results('6c8da65d-2c5e-44ef-bb0b-b8b9849aab19');
    send_test_results({
        "id": 'bffa23ee-b45a-4e9a-9405-87ab437d5cfa',
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
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{domains[www1]}}:{{ports[http][0]}}/html/semantics/embedded-content/the-iframe-element/cross_origin_child.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 370,
        "byte_start": 237,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 396,
        "byte_start": 388,
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
        "byte_end": 751,
        "byte_start": 396,
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
        "byte_end": 760,
        "byte_start": 751,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/cross_origin_parentage.sub.html"
}
```
