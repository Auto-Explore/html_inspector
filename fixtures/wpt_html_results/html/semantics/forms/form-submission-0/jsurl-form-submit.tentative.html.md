# html/semantics/forms/form-submission-0/jsurl-form-submit.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/jsurl-form-submit.tentative.html",
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

<iframe id=frameid name=framename></iframe>
<form id=formid target=framename action="resources/form.html"></form>

<script>
async_test(t => {
  frameid.src = 'resources/jsurl-form-submit-iframe.html';

  frameid.onload = t.step_func(() => {
    assert_equals(
      frameid.contentDocument.location.pathname,
      '/html/semantics/forms/form-submission-0/resources/jsurl-form-submit-iframe.html');

    frameid.onload = t.step_func_done(() => {
      assert_equals(
        frameid.contentDocument.location.pathname,
        '/html/semantics/forms/form-submission-0/resources/form.html');
    });

    frameid.contentDocument.getElementById('anchorid').click();
  });

}, `Verifies that form submissions scheduled inside javascript: urls take precedence over the javascript: url's return value.`);
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
  "source_name": "html/semantics/forms/form-submission-0/jsurl-form-submit.tentative.html"
}
```
