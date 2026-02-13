# html/semantics/forms/the-input-element/datetime-local-trailing-zeros.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/datetime-local-trailing-zeros.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel=help href="https://html.spec.whatwg.org/multipage/multipage/common-microsyntaxes.html#local-dates-and-times">
<link rel=help href="https://html.spec.whatwg.org/multipage/multipage/states-of-the-type-attribute.html#local-date-and-time-state-(type=datetime-local)">

<input id=input type=datetime-local value="2022-04-19T12:34:56.010">

<script>
test(() => {
  assert_equals(input.value, '2022-04-19T12:34:56.01');
}, 'Verifies that trailing zeros in the milliseconds portion of the date strings are removed.');
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
  "source_name": "html/semantics/forms/the-input-element/datetime-local-trailing-zeros.html"
}
```
