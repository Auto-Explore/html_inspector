# html/semantics/document-metadata/interactions-of-styling-and-scripting/script-created-style-element-does-not-block-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/interactions-of-styling-and-scripting/script-created-style-element-does-not-block-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Script-created style element is not script-blocking</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/utils.js"></script>
<script>
const style = document.createElement('style');
const sheet = document.createTextNode('@import url(stylesheet.py?delay=1);');
style.appendChild(sheet);
document.head.appendChild(style);
</script>
<h1>Some text</h1>
<script>
test(() => {
  assert_false(styleExists("h1 { color: purple; }"),
               'stylesheet should still be pending');
  const h1 = document.querySelector('h1');
  assert_equals(getComputedStyle(h1).color, 'rgb(0, 0, 0)');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/interactions-of-styling-and-scripting/script-created-style-element-does-not-block-script.html"
}
```
