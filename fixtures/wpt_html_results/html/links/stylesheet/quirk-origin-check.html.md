# html/links/stylesheet/quirk-origin-check.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/stylesheet/quirk-origin-check.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks -->
<title>Origin check for stylesheet with non-CSS MIME type quirk</title>
<link rel="stylesheet" href="data:text/plain,.test { background: red }">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p class=test>There should be no red.</p>
<script>
setup({ single_test: true });
onload = () => {
  assert_equals(getComputedStyle(document.querySelector('.test')).backgroundColor, 'rgba(0, 0, 0, 0)');
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
      "category": "Html",
      "code": "html.link.href.invalid",
      "message": "Bad value “data:text/plain,.test { background: red }” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 160,
        "byte_start": 88,
        "col": 1,
        "line": 3
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
  "source_name": "html/links/stylesheet/quirk-origin-check.html"
}
```
