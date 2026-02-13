# html/browsers/history/the-history-interface/iframe_history_go_0.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/iframe_history_go_0.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
  <title>iframe_history_go_0</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
</head>
<body>
<iframe></iframe>
<script>
promise_test(async (t) => {
  let iframe = null;
  const OLD_URL = 'blank-old.html';
  const NEW_URL = 'blank-new.html';

  await new Promise(resolve => {
    iframe = document.createElement('iframe');
    iframe.onload = () => resolve();
    iframe.src = OLD_URL;
    document.body.appendChild(iframe);
    t.add_cleanup(() => iframe.remove());
  });

  assert_equals(iframe.contentDocument.body.textContent, 'This is an old page.\n');

  await new Promise(resolve => {
    iframe.onload = () => resolve();
    iframe.src = NEW_URL;
  });

  assert_equals(iframe.contentDocument.body.textContent, 'This is a new page.\n');

  await new Promise(resolve => {
    iframe.onload = () => resolve();
    iframe.contentWindow.history.go(0);
  });

  assert_equals(iframe.contentDocument.body.textContent, 'This is a new page.\n');
}, 'iframe\'s history.go(0) performs a location.reload()');
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
  "source_name": "html/browsers/history/the-history-interface/iframe_history_go_0.html"
}
```
