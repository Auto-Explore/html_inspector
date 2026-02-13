# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation-1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Check that sandboxed iframe can perform navigation on the top frame
           when allow-top-navigation is set</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      // We are the main test page.  Open a popup, so that we can
      // can experiment navigation of the top frame.
      async_test(t => {
        window.addEventListener("message", t.step_func_done(e => {
          assert_equals(e.data, "can navigate");
          e.source.close();
        }));
        let sandbox = "allow-top-navigation allow-scripts";
        window.open("support/load-into-the-iframe.html?sandbox=" + sandbox);
      }, "Frames with `allow-top-navigation` should be able to navigate the top frame.");
    </script>
  </body>
</html>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_top_navigation-1.html"
}
```
