# html/semantics/forms/the-button-element/button-type-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-type-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#attr-button-type">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="button@type values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<button type="reset">
<button type="ReSeT">
<button type="reſet">
<button type="submit">
<button type="SuBmIt">
<button type="ſubmit">
<script>
const button = document.querySelectorAll("button");

test(() => {
  assert_equals(button[0].type, "reset", "lowercase valid");
  assert_equals(button[1].type, "reset", "mixed case valid");
  assert_equals(button[2].type, "submit", "non-ASCII invalid");
}, "keyword reset");

test(() => {
  assert_equals(button[3].type, "submit", "lowercase valid");

  // vacuous: the invalid value default is currently submit, so even if the UA
  // treats this as invalid, the observable behaviour would still be correct
  assert_equals(button[4].type, "submit", "mixed case valid");

  // vacuous: the invalid value default is currently submit, so even if the UA
  // treats this as valid, the observable behaviour would still be correct
  assert_equals(button[5].type, "submit", "non-ASCII invalid");
}, "keyword submit");
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
  "source_name": "html/semantics/forms/the-button-element/button-type-enumerated-ascii-case-insensitive.html"
}
```
