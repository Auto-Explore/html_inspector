# html/semantics/permission-element/type-supported-feature-detect.tentative.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/permission-element/type-supported-feature-detect.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Permission Element: Feature detection</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  test(() => {
    assert_not_equals(typeof HTMLPermissionElement, undefined,
        `HTMLPermissionElement should not be "undefined"`);
    assert_equals(typeof HTMLPermissionElement.isTypeSupported, "function",
        `The type of the isTypeSupported should be "function"`);
  }, "Test HTMLPermissionElement and isTypeSupported existence");

  [
    "geolocation",
    "camera",
    "microphone",
    " camera  microphone ",
    " microphone  camera   ",
    " camera camera microphone",
    "microphone microphone camera "
  ].forEach((type) => {
    test(() => {
      assert_true(HTMLPermissionElement.isTypeSupported(type));
    }, `Test HTMLPermissionElement should support type ${type}`);
  });

  [
    "invalid",
    "cameraa",
    "microphone geolocation",
    "camera geolocation",
    "camera geolocation microphone",
  ].forEach((type) => {
    test(() => {
      assert_false(HTMLPermissionElement.isTypeSupported(type));
    }, `Test HTMLPermissionElement should not support type ${type}`);
  });
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “body”.",
      "severity": "Error",
      "span": {
        "byte_end": 1238,
        "byte_start": 1231,
        "col": 1,
        "line": 41
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
  "source_name": "html/semantics/permission-element/type-supported-feature-detect.tentative.html"
}
```
