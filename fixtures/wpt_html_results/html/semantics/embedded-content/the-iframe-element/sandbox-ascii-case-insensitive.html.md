# html/semantics/embedded-content/the-iframe-element/sandbox-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/sandbox-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>iframe 'sandbox' ASCII case insensitive</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async_test(function(t) {
  let iframe = document.createElement('iframe');
  iframe.setAttribute('sandbox', 'allow-same-or\u0130gin');
  iframe.setAttribute('hidden', '');

  assert_true(iframe.sandbox.supports('allow-same-origin'), 'supports the allow-same-origin token');

  iframe.src = 'support/blank.htm';
  iframe.onload = t.step_func_done(function() {
    try {
      assert_equals(iframe.contentDocument, null, 'child document not reachable');
    } catch (e) {
      // The assert_equals throwing is a pass.
    }
  });
  document.body.appendChild(iframe);
}, document.title + ', allow-same-or\u0130gin');

async_test(function(t) {
  let iframe = document.createElement('iframe');
  iframe.setAttribute('sandbox', 'allow-\u017Fcripts');
  iframe.setAttribute('hidden', '');

  assert_true(iframe.sandbox.supports('allow-scripts'), 'supports the allow-scripts token');

  window.onmessage = t.unreached_func('no scripts should run in the iframe');
  iframe.src = 'support/sandbox_allow_script.html';
  iframe.onload = t.step_func(function() {
    t.step_timeout(t.step_func_done(), 100);
  });
  document.body.appendChild(iframe);
}, document.title + ', allow-\u017Fcripts');
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/sandbox-ascii-case-insensitive.html"
}
```
