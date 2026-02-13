# html/rendering/non-replaced-elements/the-page/iframe-marginwidth-marginheight.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-page/iframe-marginwidth-marginheight.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>iframe marginwidth and marginheight</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe src="/common/blank.html" marginwidth=0 marginheight=0></iframe>
<script>
setup({ single_test: true });
onload = () => {
  assert_equals(window[0].document.body.attributes.length, 0, "Number of attributes on the child document's body");
  done();
}
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
  "source_name": "html/rendering/non-replaced-elements/the-page/iframe-marginwidth-marginheight.html"
}
```
