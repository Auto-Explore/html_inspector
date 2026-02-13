# html/semantics/document-metadata/interactions-of-styling-and-scripting/style-element-media-not-match-does-not-block-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/interactions-of-styling-and-scripting/style-element-media-not-match-does-not-block-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Style element is not script-blocking when media doesn't match</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/utils.js"></script>
<style media="print">
@import url('stylesheet.py?delay=1');
</style>
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
  "source_name": "html/semantics/document-metadata/interactions-of-styling-and-scripting/style-element-media-not-match-does-not-block-script.html"
}
```
