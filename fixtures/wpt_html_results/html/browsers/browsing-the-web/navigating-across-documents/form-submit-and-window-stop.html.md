# html/browsers/browsing-the-web/navigating-across-documents/form-submit-and-window-stop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/form-submit-and-window-stop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The planned navigation of a form should be cancelled by a window stop
  in the same task as the form submission</title>
<link rel="help" href="https://github.com/whatwg/html/issues/3730">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>

<body>
<script>
"use strict";
async_test(t => {
  let unloaded = false;
  window.onload = t.step_func(() => {
    form.submit();
    window.stop();
  });
  window.onunload = () => {
    unloaded = true;
  };
  t.step_timeout(() => {
    // The document should not have unloaded.
    assert_equals(unloaded, false);
    t.done();
  }, 100);
});
</script>
<form id="form">
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/form-submit-and-window-stop.html"
}
```
