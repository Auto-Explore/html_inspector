# html/semantics/forms/form-submission-0/request-submit-activation.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/request-submit-activation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="./resources/targetted-form.js"></script>
<body>
<script>
promise_test(async () => {
  let form = populateForm('<input type=submit name=n1 value=v1><button type=submit name=n2 value=v2></button>');
  let submitter = form.querySelector('button');
  let iframe = form.previousSibling;
  let event;
  form.requestSubmit(submitter);
  await loadPromise(iframe);
  assert_true(iframe.contentWindow.location.search.indexOf('n1=v1') == -1, "n1=v1");
  assert_true(iframe.contentWindow.location.search.indexOf('n2=v2') > 0), "n2=v2";
}, 'Test activation of submitter for requestSubmit');

promise_test(async () => {
  let form = populateForm('<input type=submit name=n1 value=v1><button type=submit name=n2 value=v2></button>');
  let submitter = form.querySelector('input');
  let iframe = form.previousSibling;
  let event;
  form.requestSubmit(submitter);
  await loadPromise(iframe);
  assert_true(iframe.contentWindow.location.search.indexOf('n1=v1') > 0, "n1=v1");
  assert_true(iframe.contentWindow.location.search.indexOf('n2=v2') == -1), "n2=v2";
}, 'Test activation of submitter for requestSubmit 2');
</script>
</body>
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
  "source_name": "html/semantics/forms/form-submission-0/request-submit-activation.html"
}
```
