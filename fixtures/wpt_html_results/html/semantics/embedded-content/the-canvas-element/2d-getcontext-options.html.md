# html/semantics/embedded-content/the-canvas-element/2d-getcontext-options.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-canvas-element/2d-getcontext-options.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Options conversion for getContext("2d")</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(() => {
  const expected = [
    "alpha",
    "colorSpace",
    "colorSpace toString",
    "desynchronized",
    "willReadFrequently",
  ];
  var actual = [];
  const options = {
    get alpha() {
      actual.push("alpha");
      return true;
    },
    get willReadFrequently() {
      actual.push("willReadFrequently");
      return false;
    },
    get desynchronized() {
      actual.push("desynchronized");
      return false;
    },
    get colorSpace() {
      actual.push("colorSpace");
      return {
        toString() {
          actual.push("colorSpace toString");
          return "srgb";
        }
      };
    },
  };

  const canvas = document.createElement("canvas");
  const context = canvas.getContext('2d', options);
  assert_not_equals(context, null, "context");
  assert_array_equals(actual, expected, "order of operations (creation)");
  actual = [];
  assert_equals(canvas.getContext('2d', options), context, "cached context");
  assert_array_equals(actual, expected, "order of operations (caching)");
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
  "source_name": "html/semantics/embedded-content/the-canvas-element/2d-getcontext-options.html"
}
```
