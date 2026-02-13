# html/semantics/forms/form-submission-0/form-double-submit-multiple-targets.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-double-submit-multiple-targets.html",
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

<form id=myform name=myform action="/formaction.html"></form>
<iframe id=frame1 name=target1></iframe>
<iframe id=frame2 name=target2></iframe>
<iframe id=frame3 name=target3></iframe>

<script>

promise_test(async () => {
  const frame1LoadPromise = new Promise(resolve => frame1.onload = resolve);
  const frame2LoadPromise = new Promise(resolve => frame2.onload = resolve);
  const frame3LoadPromise = new Promise(resolve => frame3.onload = resolve);

  myform.target = 'target1';
  myform.submit();
  myform.target = 'target2';
  myform.submit();
  myform.target = 'target3';
  myform.submit();

  await Promise.all([frame1LoadPromise, frame2LoadPromise, frame3LoadPromise]);

  assert_equals(frame1.contentDocument.location.pathname, '/formaction.html');
  assert_equals(frame2.contentDocument.location.pathname, '/formaction.html');
  assert_equals(frame3.contentDocument.location.pathname, '/formaction.html');

}, 'Verifies that one form used to target multiple frames in succession navigates all of them.');

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
  "source_name": "html/semantics/forms/form-submission-0/form-double-submit-multiple-targets.html"
}
```
