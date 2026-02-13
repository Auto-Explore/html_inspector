# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_escaping-3.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_escaping-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Check that popups from a sandboxed iframe escape the sandbox if
       allow-popups-to-escape-sandbox is used</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe sandbox="allow-scripts allow-popups allow-popups-to-escape-sandbox">
</iframe>
<script>
  var t = async_test();
  var ourOrigin;
  onmessage = t.step_func(function(e) {
    assert_equals(e.data, "hello", "This is our origin getter message");
    ourOrigin = e.origin;

    onmessage = t.step_func_done(function(e) {
      assert_equals(e.origin, "null", "It came from a sandboxed iframe");
      assert_equals(e.data.data, undefined, "Should have the right message");
      assert_equals(e.data.origin, ourOrigin, "Should have escaped the sandbox");
    });

    document.querySelector("iframe").src = "iframe_sandbox_popups_helper-3.html";
  });
  postMessage("hello", "*");
</script>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_popups_escaping-3.html"
}
```
