# html/semantics/scripting-1/the-script-element/module/modulepreload-referrer-check.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/modulepreload-referrer-check.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Modulepreload Referrer Header Check</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
// Initialize the window.referrers object that will be used by echo-referrer.py
window.referrers = {};
</script>
</head>
<body>
<iframe id="test-frame"></iframe>

<script>
// This test uses a simple approach to check if modulepreload sends a referrer header
promise_test(async t => {
  const importId = Date.now();
  const preloadId = Date.now() + 1;

  // Import will set window.referrers[importId] to the referrer value
  await import(`/preload/resources/echo-referrer.py?uid=${importId}`);

  const link = document.createElement('link');
  link.rel = 'modulepreload';
  link.href = `/preload/resources/echo-referrer.py?uid=${preloadId}`;

  // Wait for the preload to complete using onload event
  const preloadComplete = new Promise(resolve => {
    link.onload = resolve;
    link.onerror = () => {
      assert_unreached("Modulepreload failed to load");
    };
  });

  document.head.appendChild(link);
  await preloadComplete;

  // Second import ensures the module is loaded and referrer is recorded
  await import(`/preload/resources/echo-referrer.py?uid=${preloadId}`);

  // Check if referrers were recorded in the global window.referrers object
  assert_equals(window.referrers[importId], location.href, "Dynamic import should have a referrer header");
  assert_equals(window.referrers[preloadId], location.href, "Modulepreload should have a referrer header");

}, "Modulepreload should send a referrer header");
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/modulepreload-referrer-check.html"
}
```
