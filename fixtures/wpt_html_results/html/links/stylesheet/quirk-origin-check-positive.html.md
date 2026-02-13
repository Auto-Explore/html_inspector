# html/links/stylesheet/quirk-origin-check-positive.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/stylesheet/quirk-origin-check-positive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks -->
<title>Same-origin stylesheet with non-CSS MIME type quirk</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#link-type-stylesheet">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="stylesheet" href="resources/quirk-stylesheet.css.txt">
<p class="test">This text should be green.</p>
<script>
setup({ single_test: true });
onload = () => {
  assert_equals(
    getComputedStyle(document.querySelector('.test')).color,
    'rgb(0, 128, 0)',
    "Same-origin stylesheet with non-CSS MIME type should be applied in quirks mode"
  );
  done();
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 23,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/links/stylesheet/quirk-origin-check-positive.html"
}
```
