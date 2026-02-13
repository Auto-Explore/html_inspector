# html/semantics/embedded-content/the-iframe-element/change_parentage.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/change_parentage.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Change the frame heriarchy</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
  <iframe src="change_child.html"></iframe>
</body>
<script>
    async_test(function(t) {
        var timer = window.setInterval(t.step_func(poll), 100);
        function poll() {
            // We wait for the grandchild's script to set the custom attribtue.
            // Note that if this test passes, the grandchild's script must have been run twice,
            // once to trigger the move from the child to here, and once to pass this test.
            if (document.body.getAttribute("data-contains-grandchild")) {
                window.clearTimeout(timer);
                t.done();
            }
        }
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
        "byte_end": 254,
        "byte_start": 246,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 818,
        "byte_start": 254,
        "col": 9,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 827,
        "byte_start": 818,
        "col": 1,
        "line": 22
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/change_parentage.html"
}
```
