# html/semantics/forms/form-submission-0/form-submit-iframe-then-location-navigate.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-submit-iframe-then-location-navigate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- This behavior is not explicitly specified. -->

<iframe id=myframe name=framename></iframe>
<form id=myform target=framename action="resources/form.html"></form>

<script>
async_test(t => {
  myframe.onload = t.step_func_done(() => {
    assert_equals(
      myframe.contentDocument.location.pathname,
      '/html/semantics/forms/form-submission-0/resources/location.html');
  });
  myform.submit();
  myframe.contentDocument.location = 'resources/location.html';
}, 'Verifies that location navigations take precedence when following form submissions.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/form-submission-0/form-submit-iframe-then-location-navigate.html"
}
```
