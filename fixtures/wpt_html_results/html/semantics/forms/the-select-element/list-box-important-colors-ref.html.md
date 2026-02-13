# html/semantics/forms/the-select-element/list-box-important-colors-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/list-box-important-colors-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<style>
option:not(:checked) {
  background-color: green;
  color: blue;
}
</style>

<select multiple size=4 autofocus>
  <option>option</option>
  <option id=target>focused and checked</option>
</select>

<script>
const target = document.getElementById('target');
(async () => {
  await (new test_driver.Actions()
    .pointerMove(0, 0, {origin: target})
    .pointerDown()
    .pointerUp())
    .send();
  document.documentElement.classList.remove('reftest-wait');
})();
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
  "source_name": "html/semantics/forms/the-select-element/list-box-important-colors-ref.html"
}
```
