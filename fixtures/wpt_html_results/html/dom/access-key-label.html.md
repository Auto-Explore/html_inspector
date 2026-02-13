# html/dom/access-key-label.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/access-key-label.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>accessKeyLabel attribute</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
function createButtonWithAccessKey(accessKey) {
    const button = document.createElement('button');
    button.setAttribute('accesskey', accessKey);
    return button;
}

// The modifiers are not the same across all browser vendors.
// Therefore, the test uses non empty.
test(() => {
    const element = createButtonWithAccessKey('b');
    assert_not_equals(element.accessKeyLabel, '');
}, 'Returns non empty string when accesskey is valid');

test(() => {
    const element = createButtonWithAccessKey('s 0');
    assert_equals(element.accessKeyLabel, '');
}, 'Returns empty string when accesskey is invalid');
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 840,
        "byte_start": 833,
        "col": 1,
        "line": 27
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
  "source_name": "html/dom/access-key-label.html"
}
```
