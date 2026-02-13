# html/rendering/widgets/input-text-size.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-text-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://html.spec.whatwg.org/C/#the-input-element-as-a-text-entry-widget">
<title>Test `size` attribute behavior</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>

<input id="missing">
<input id="invalid" size="-1">
<input id="size0" size="0">
<input id="size1" size="1">
<input id="size10" size="10">
<input id="size20" size="20">
<input id="size21" size="21">
<input id="computed" style="border:none; padding:0;" size="19">
<input id="computedNone" style="display:none" size="17">

<script>
test(() => {
  assert_equals(missing.offsetWidth, size20.offsetWidth);
}, 'A misssing attribute is equivalent to size=20');

test(() => {
  assert_equals(invalid.offsetWidth, size20.offsetWidth, 'size="-1"');
  assert_equals(size0.offsetWidth, size20.offsetWidth, 'size="0"');
}, 'An invalid attribute value is equivalent to size=20');

test(() => {
  assert_less_than(size1.offsetWidth, size10.offsetWidth, '1 < 10');
  assert_less_than(size10.offsetWidth, size20.offsetWidth, '10 < 20');
  assert_less_than(size20.offsetWidth, size21.offsetWidth, '20 < 21');
}, 'The width depends on a size attribute');

test(() => {
  const computedString = getComputedStyle(computed).width;
  assert_equals(computed.offsetWidth,
      Math.round(computedString.substring(0, computedString.length - 2)));
}, 'Size attribute value affects layout-dependent computed style');

test(() => {
  const computedString = getComputedStyle(computedNone).width;
  assert_equals(computedString, 'auto');
}, 'Size attribute value is not a presentational hint');
</script>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.size.invalid",
      "message": "Bad value “-1” for attribute “size” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 325,
        "byte_start": 295,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.input.size.invalid",
      "message": "Bad value “0” for attribute “size” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 353,
        "byte_start": 326,
        "col": 1,
        "line": 10
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
  "source_name": "html/rendering/widgets/input-text-size.html"
}
```
