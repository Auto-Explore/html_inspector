# html/links/stylesheet/quirk-origin-check-recursive-import.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/links/stylesheet/quirk-origin-check-recursive-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks -->
<title>Origin check for stylesheet with non-CSS MIME type quirk: recursive @import</title>
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1477672">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1850965">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
setup({ single_test: true });
let errors = 0;
onload = () => {
  assert_equals(errors, 1);
  done();
};
</script>
<link rel="stylesheet" href="data:/,@import url('x/');" onerror="errors++">
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
      "message": "Bad value “data:/,@import url('x/');” for attribute “href” on element “link”.",
      "severity": "Warning",
      "span": {
        "byte_end": 576,
        "byte_start": 501,
        "col": 1,
        "line": 15
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
  "source_name": "html/links/stylesheet/quirk-origin-check-recursive-import.html"
}
```
