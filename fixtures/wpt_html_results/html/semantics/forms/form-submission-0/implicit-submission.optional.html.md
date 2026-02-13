# html/semantics/forms/form-submission-0/implicit-submission.optional.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/implicit-submission.optional.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#implicit-submission">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="./resources/targetted-form.js"></script>
<body>
<script>
// This test file is "optional" because triggering implicit submission by
// "Enter" key is not standardized.

const ENTER = '\uE007';

promise_test(async () => {
  let form = populateForm('<input name=text value=abc><input name=submitButton type=submit>');
  let event;
  form.text.focus();
  form.addEventListener('submit', e => { event = e; });
  await test_driver.send_keys(form.text, ENTER);
  assert_true(event.bubbles);
  assert_true(event.cancelable);
  assert_equals(event.submitter, form.submitButton);
  assert_true(event instanceof SubmitEvent);
}, 'Submit event with a submit button');

promise_test(async () => {
  let form = populateForm('<input name=text value=abc>');
  let event;
  form.text.focus();
  form.addEventListener('submit', e => { event = e; });
  await test_driver.send_keys(form.text, ENTER);
  assert_true(event.bubbles);
  assert_true(event.cancelable);
  assert_equals(event.submitter, null);
  assert_true(event instanceof SubmitEvent);
}, 'Submit event with no submit button');

promise_test(async (test) => {
  let form = populateForm('<input name=text value=abc><input name=submitButton type=submit disabled>');
  form.text.focus();
  form.addEventListener('submit', test.unreached_func('submit event should not be dispatched'));
  await test_driver.send_keys(form.text, ENTER);
}, 'Submit event with disabled submit button');

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
  "source_name": "html/semantics/forms/form-submission-0/implicit-submission.optional.html"
}
```
