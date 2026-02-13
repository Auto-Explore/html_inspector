# html/semantics/forms/form-submission-0/form-double-submit-default-action.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-double-submit-default-action.html",
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
  The submit() in event handler should get superseded by the default action
  submit(), which isn't preventDefaulted. This is per the Form Submission
  Algorithm [1], step 24, which says that new planned navigations replace old
  planned navigations.
  [1] https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm
-->
<body>
<script>
promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  submitter.addEventListener('click', () => {
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v4");
}, 'default submit action should supersede input onclick submit()');

promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><button>submit</button>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("button");
  submitter.addEventListener('click', (e) => {
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v4");
}, 'default submit action should supersede button onclick submit()');

promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  form.addEventListener('click', () => {
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v4");
}, 'default submit action should supersede form onclick submit()');

promise_test(async () => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  form.addEventListener('submit', () => {
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v4");
}, 'default submit action should supersede form onsubmit submit()');

promise_test(async (t) => {
  let form = populateForm('<input name=n1 value=v1><input type=submit>');
  let iframe = form.previousSibling;
  let input = form.querySelector("input[name=n1]");
  let submitter = form.querySelector("input[type=submit]");
  form.addEventListener('click', () => {
    input.value = 'v2';
    form.submit();
    input.value = 'v3';
    form.submit();
    input.value = 'v4';
  });
  form.addEventListener('submit', () => {
    input.value = 'v5';
    form.submit();
    input.value = 'v6';
    form.submit();
    input.value = 'v7';
  });
  submitter.click();
  await loadPromise(iframe);
  assert_equals(getParamValue(iframe, "n1"), "v7");
}, 'default submit action should supersede form onclick/onsubmit submit()');
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
  "source_name": "html/semantics/forms/form-submission-0/form-double-submit-default-action.html"
}
```
