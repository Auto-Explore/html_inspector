# html/semantics/embedded-content/the-iframe-element/iframe-synchronously-discard.html

Counts:
- errors: 5
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-synchronously-discard.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>IFrame discards are processed synchronously</title>
<body></body>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  async_test(function(t) {
    var child = document.createElement("iframe");
    child.src = "support/blank.htm?1";
    child.onload = t.step_func(function () {
      var childWindow = child.contentWindow;
      var grandchild = childWindow.document.createElement("iframe");
      grandchild.src = "blank.htm?2";
      grandchild.onload = t.step_func(function () {
        var grandchildWindow = grandchild.contentWindow;
        assert_equals(child.contentWindow, childWindow, "child window");
        assert_equals(childWindow.parent, window, "child parentage");
        assert_equals(grandchild.contentWindow, grandchildWindow, "grandchild window");
        assert_equals(grandchildWindow.parent, childWindow, "grandchild parentage");
        document.body.removeChild(child);
        assert_equals(child.contentWindow, null, "child should be discarded");
        assert_equals(childWindow.parent, null, "child window should be discarded");
        assert_equals(grandchild.contentWindow, null, "grandchild should be discarded");
        assert_equals(grandchildWindow.parent, null, "grandchild window should be discarded");
        t.done();
      });
      childWindow.document.body.appendChild(grandchild);
    });
    document.body.appendChild(child);
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
        "byte_end": 150,
        "byte_start": 110,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 159,
        "byte_start": 150,
        "col": 41,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 215,
        "byte_start": 206,
        "col": 47,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1490,
        "byte_start": 224,
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
        "byte_end": 1499,
        "byte_start": 1490,
        "col": 1,
        "line": 32
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-synchronously-discard.html"
}
```
