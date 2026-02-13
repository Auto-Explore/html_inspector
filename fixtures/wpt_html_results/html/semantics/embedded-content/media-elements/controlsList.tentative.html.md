# html/semantics/embedded-content/media-elements/controlsList.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/controlsList.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test controlsList attribute</title>
<link rel="help" href="https://github.com/whatwg/html/pull/6715">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
const allowedValues = [
  "nodownload",
  "nofullscreen",
  "noplaybackrate",
  "noremoteplayback",
];

function testControlsList(tagName) {
  const element = document.createElement(tagName);

  // Test that supports() is returning true for allowed values.
  for (const value of allowedValues) {
    assert_true(
      element.controlsList.supports(value),
      `tag = ${element.tagName}, value = ${value} must be supported`
    );
  }
}

["audio", "video"].forEach((tagName) => {
  test(
    () => testControlsList(tagName),
    `Test controlsList allowed values for <${tagName}>`
  );
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
  "source_name": "html/semantics/embedded-content/media-elements/controlsList.tentative.html"
}
```
