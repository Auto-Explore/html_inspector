# html/semantics/embedded-content/the-iframe-element/content_document_changes_only_after_load_matures.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/content_document_changes_only_after_load_matures.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Iframe's contentDocument should only change after its pending load has matured.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body></body>
<script>
async_test(function(t) {
    var iframe = document.createElement("iframe");
    document.body.appendChild(iframe);
    var checkedDuringParse = false;
    iframe.onload = t.step_func_done(function() {
        testContentDocument();
        assert_true(checkedDuringParse);
    });

    let url = "support/iframe-that-checks-contentDocument.html";
    window.testContentDocument = t.step_func(function() {
        assert_true(iframe.contentDocument.location.toString().includes(url));
        checkedDuringParse = true;
    });

    assert_equals(iframe.contentDocument.location.toString(), "about:blank");
    iframe.src = url + "?pipe=trickle(d2)";
    // The location of the contentDocument should not change until the new document has matured.
    assert_equals(iframe.contentDocument.location.toString(), "about:blank");
}, "contentDocument should only change after a load matures.");
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
        "byte_end": 262,
        "byte_start": 254,
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
        "byte_end": 1152,
        "byte_start": 262,
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
        "byte_end": 1161,
        "byte_start": 1152,
        "col": 1,
        "line": 28
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/content_document_changes_only_after_load_matures.html"
}
```
