# html/canvas/element/global-hdr-headroom/attribute.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/global-hdr-headroom/attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  // Test setting, getting, and default values of globalHDRHeadroom
  const canvas = new OffscreenCanvas(64, 64);
  const ctx = canvas.getContext('2d');
  if (ctx.globalHDRHeadroom !== undefined) {
    // Default value is 0.
    assert_equals(ctx.globalHDRHeadroom, 0);

    // Negative and NaN values are rejected.
    ctx.globalHDRHeadroom = -1.0;
    assert_equals(ctx.globalHDRHeadroom, 0);
    ctx.globalHDRHeadroom = NaN;
    assert_equals(ctx.globalHDRHeadroom, 0);
    ctx.globalHDRHeadroom = -Infinity;
    assert_equals(ctx.globalHDRHeadroom, 0);

    // Nonnegative values are not rejected.
    ctx.globalHDRHeadroom = 1;
    assert_equals(ctx.globalHDRHeadroom, 1);
    ctx.globalHDRHeadroom = 0;
    assert_equals(ctx.globalHDRHeadroom, 0);

    // Infinity is a valid value.
    ctx.globalHDRHeadroom = Infinity;
    assert_equals(ctx.globalHDRHeadroom, Infinity);

    // Strings from dynamic-range-limit are rejected.
    ctx.globalHDRHeadroom = 'constrained';
    assert_equals(ctx.globalHDRHeadroom, Infinity);
  }
});
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
  "source_name": "html/canvas/element/global-hdr-headroom/attribute.html"
}
```
