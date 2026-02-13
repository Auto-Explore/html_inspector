# html/semantics/forms/form-submission-0/jsurl-navigation-then-form-submit.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/jsurl-navigation-then-form-submit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">

<!-- The expected behavior of this test is not explicitly specified. -->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
async_test(t => {
  window.onload = t.step_func(() => {
    const iframe = document.createElement('iframe');
    iframe.name = 'myframe';

    iframe.onload = t.step_func_done(() => {
      assert_equals(iframe.contentDocument.location.pathname, '/formaction.html');
    });

    const form = document.createElement('form');
    form.target = iframe.name;
    form.action = '/formaction.html';
    document.body.appendChild(form);

    iframe.src = 'javascript:false';
    document.body.appendChild(iframe);
    form.submit();
  });
}, 'Verifies that form submissions cancel javascript navigations to prevent duplicate load events.');
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
  "source_name": "html/semantics/forms/form-submission-0/jsurl-navigation-then-form-submit.html"
}
```
