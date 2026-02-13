# html/semantics/forms/the-form-element/form-action-submission.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-action-submission.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>form action="" attribute effect on submission</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#dom-fs-action">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

// promise_test instead of async_test because all tests use window.success, and so can't run at the same time.

promise_test(t => {
  return new Promise(resolve => {
    window.success = t.step_func(locationLoaded => {
      const expected = (new URL("resources/target/form-action-url-target.html?name=value", location.href)).href;
      assert_equals(locationLoaded, expected);
      resolve();
    });

    const iframe = document.createElement("iframe");
    iframe.src = "resources/form-with-action.sub.html?action=target/form-action-url-target.html";
    document.body.appendChild(iframe);
  });
}, "An action URL should be resolved relative to the document's base URL (= document's URL in this case)");

promise_test(t => {
  return new Promise(resolve => {
    window.success = t.step_func(locationLoaded => {
      const expected = (new URL("resources/form-with-action.sub.html?name=value", location.href)).href;
      assert_equals(locationLoaded, expected);
      resolve();
    });

    const iframe = document.createElement("iframe");
    iframe.src = "resources/form-with-action.sub.html?action=";
    document.body.appendChild(iframe);
  });
}, "An empty-string action should submit the form to the document's URL (= document's base URL in this case)");

promise_test(t => {
  return new Promise(resolve => {
    window.success = t.step_func(locationLoaded => {
      const expected = (new URL("resources/form-no-action.html?name=value", location.href)).href;
      assert_equals(locationLoaded, expected);
      resolve();
    });

    const iframe = document.createElement("iframe");
    iframe.src = "resources/form-no-action.html";
    document.body.appendChild(iframe);
  });
}, "A missing action should submit the form to the document's URL (= document's base URL in this case)");
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
  "source_name": "html/semantics/forms/the-form-element/form-action-submission.html"
}
```
