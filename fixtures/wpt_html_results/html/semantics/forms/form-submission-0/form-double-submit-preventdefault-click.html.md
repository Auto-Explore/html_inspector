# html/semantics/forms/form-submission-0/form-double-submit-preventdefault-click.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-double-submit-preventdefault-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="./resources/targetted-form.js"></script>
<!--
  The submit() in event handler should *not* get superseded in this case by the
  default action submit(), because event handler here calls preventDefault().
-->
<body>
<script>
promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  submitter.addEventListener('click', (e) => {
    e.preventDefault();
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v3");
}, 'PreventDefaulting input onclick should allow submit() to succeed');

promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><button>submit</button>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("button");
  submitter.addEventListener('click', (e) => {
    e.preventDefault();
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v3");
}, 'PreventDefaulting button onclick should allow submit() to succeed');

promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  form.addEventListener('click', (e) => {
    e.preventDefault();
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v3");
}, 'PreventDefaulting form onclick should allow submit() to succeed');
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
  "source_name": "html/semantics/forms/form-submission-0/form-double-submit-preventdefault-click.html"
}
```
