# html/semantics/embedded-content/the-iframe-element/sandbox_002.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox_002.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>HTML5 Sandbox: Allow autoplay for HTML5 Video inside iframe with sandbox attribute if sandbox='allow-scripts'.</title>
    <meta name=timeout content=long>
    <meta content="text/html; charset=UTF-8" http-equiv="Content-Type" />
    <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <script>
      async_test(function (t) {
        var callback = t.step_func_done(function(event) {
          assert_true('sandbox' in document.createElement('iframe'));
          assert_equals(event.data, "play event fired");
        });

        window.addEventListener("message", callback, false);
      }, "Allow autoplay for HTML5 Video inside iframe with sandbox attribute if sandbox='allow-scripts'.");
    </script>
    <iframe src="support/iframe_sandbox_002.htm" sandbox="allow-scripts" style="display: none"></iframe>
    <div id=log></div>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox_002.htm"
}
```
