# html/semantics/forms/the-select-element/customizable-select/select-listitems-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-listitems-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=test-wait>
<link rel=help href="https://issues.chromium.org/issues/396475564">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<script>
const select = document.createElement('select');
document.documentElement.appendChild(select);
const div = document.createElement('div');
select.appendChild(div);
const optgroup = document.createElement('optgroup');
div.appendChild(optgroup);

(async () => {
  await test_driver.bless();
  document.querySelector('select').showPicker();
  document.documentElement.classList.remove('test-wait');
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-listitems-crash.html"
}
```
