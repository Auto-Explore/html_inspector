# html/semantics/forms/the-input-element/input-type-change-submit.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-type-change-submit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#the-input-element">
<link rel="help" href="https://dom.spec.whatwg.org/#eventtarget-activation-behavior">
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#input-activation-behavior">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="../form-submission-0/resources/targetted-form.js"></script>
<body>
<script>
["text", "search", "tel", "url", "email", "password", "date", "month", "week",
 "time", "datetime-local", "number", "range", "color", "checkbox", "radio", "file",
 "image", "reset", "button"].forEach(function(type) {
  promise_test(async (t) => {
    let form = populateForm(`<input type=${type} name=n1 value=v1>`);
    let iframe = form.previousSibling;
    t.add_cleanup(() => {
      form.remove();
      iframe.remove();
    });

    let input = form.querySelector("input[name=n1]");
    input.addEventListener('click', () => {
      input.type = "submit";
      input.value = 'v2';
    });
    input.click();
    await loadPromise(iframe);
    assert_equals(getParamValue(iframe, "n1"), "v2");

    // cleanup
    form.remove();
    iframe.remove();
  }, `default submit action when the input type is changed from ${type} to submit`);
});
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
  "source_name": "html/semantics/forms/the-input-element/input-type-change-submit.html"
}
```
