# html/semantics/forms/the-input-element/type-change-file-to-text-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/type-change-file-to-text-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#input-type-change">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<input id="myInput" type="file">
<script>
  test(() => {
    myInput.offsetTop;
    myInput.type = "text";
  }, "Changing type from file to text should not crash.");
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
  "source_name": "html/semantics/forms/the-input-element/type-change-file-to-text-crash.html"
}
```
