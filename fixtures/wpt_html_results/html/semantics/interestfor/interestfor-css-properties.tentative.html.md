# html/semantics/interestfor/interestfor-css-properties.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-css-properties.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/css/support/parsing-testcommon.js"></script>
<script src="/css/support/computed-testcommon.js"></script>
<script src="/css/support/interpolation-testcommon.js"></script>

<div id=target></div>
<div id=scratch></div>

<script>
function testprop(prop) {
  // Computed values:
  test_computed_value(prop, '0s');
  test_computed_value(prop, '0ms', '0s');
  test_computed_value(prop, '32s');
  test_computed_value(prop, '123ms', '0.123s');
  test_computed_value(prop, 'normal', 'normal');

  // Valid values:
  test_valid_value(prop, '0s');
  test_valid_value(prop, '0ms');
  test_valid_value(prop, '32s');
  test_valid_value(prop, '123ms');
  test_valid_value(prop, 'inherit');
  test_valid_value(prop, 'calc(2s * sibling-index())');
  test_valid_value(prop, 'normal');

  // Invalid values:
  test_invalid_value(prop, '0', '0s');
  test_invalid_value(prop, 'foo');
  test_invalid_value(prop, '-1s');
  test_invalid_value(prop, 'none');
  test_invalid_value(prop, 'auto');

  // Animations:
  test_interpolation({
    property: prop,
    from: '1s',
    to: '2000ms',
  }, [
    {at: -1.5, expect: '0s'}, // Clamping at 0
    {at: -0.3, expect: '0.7s'},
    {at: 0, expect: '1s'},
    {at: 0.5, expect: '1.5s'},
    {at: 1, expect: '2s'},
    {at: 1.5, expect: '2.5s'},
  ]);
}

testprop('interest-delay-start');
testprop('interest-delay-end');
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
  "source_name": "html/semantics/interestfor/interestfor-css-properties.tentative.html"
}
```
