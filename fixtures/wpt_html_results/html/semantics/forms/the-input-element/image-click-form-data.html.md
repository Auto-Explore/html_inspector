# html/semantics/forms/the-input-element/image-click-form-data.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/image-click-form-data.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Check form-data for image submit button with non-empty 'value' attribute</title>
<link rel="author" title="Shanmuga Pandi" href="mailto:shanmuga.m@samsung.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#constructing-form-data-set">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

// promise_test instead of async_test because this test use window.success, and so can't run at the same time.

promise_test(t => {
  return new Promise(resolve => {
    window.success = t.step_func(locationLoaded => {
      const expected = (new URL("resources/image-submit-click.html?name.x=0&name.y=0", location.href)).href;
      assert_equals(locationLoaded, expected);
      resolve();
    });

    const iframe = document.createElement("iframe");
    iframe.src = "resources/image-submit-click.html";
    document.body.appendChild(iframe);
  });
}, "Image submit button should not add extra form data if 'value' attribute is present with non-empty value");
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
  "source_name": "html/semantics/forms/the-input-element/image-click-form-data.html"
}
```
