# html/semantics/embedded-content/the-iframe-element/iframe_remove_src.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_remove_src.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test that removing the src attribute of an iframe loads about:blank
  instead of whatever was loaded previously.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
  <script>
    var iframe;
    var t = async_test();
    t.step(setupFrame);

    function setupFrame() {
      iframe = document.createElement("iframe");
      iframe.src = URL.createObjectURL(new Blob(["text"], { type: "text/html" }));
      iframe.onload = t.step_func(blobLoaded);
      document.body.appendChild(iframe);
    }

    var removalRunning = false;
    function blobLoaded() {
      assert_equals(iframe.contentDocument.location.protocol, "blob:",
                    "Should have loaded the blob");
      assert_equals(iframe.contentDocument.documentElement.textContent, "text",
                    "Should have loaded the blob text");
      iframe.onload = t.step_func_done(aboutBlankLoaded);
      removalRunning = true;
      iframe.removeAttribute("src");
      removalRunning = false;
    }

    function aboutBlankLoaded() {
      assert_false(removalRunning, "Should not have loaded about:blank sync");
      assert_equals(iframe.contentDocument.location.href, "about:blank",
                    "Should have loaded about:blank");
      assert_equals(iframe.contentDocument.documentElement.textContent, "",
                    "Should have loaded the about:blank text");
    }
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_remove_src.html"
}
```
