# html/rendering/non-replaced-elements/form-controls/input-line-height-computed.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/input-line-height-computed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>used value and computed value of 'line-height' on input elements as text entry widgets</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-input-element-as-a-text-entry-widget">
<link rel="help" href="https://drafts.csswg.org/cssom/#resolved-values">
<link rel="help" href="https://drafts.css-houdini.org/css-typed-om/#computed-stylepropertymapreadonly-objects">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  input { line-height: 1px; }
</style>
<p><input type=text value=text>
<p><input type=tel value=tel>
<p><input type=search value=search>
<p><input type=url value=url>
<p><input type=email value=email>
<p><input type=password value=password></p>
<script>
const inputs = document.querySelectorAll('input');
for (const input of inputs) {
  test(() => {
    const usedLineHeight = getComputedStyle(input).lineHeight;
    assert_not_equals(usedLineHeight, '1px', 'usedLineHeight');
    assert_not_equals(usedLineHeight, 'normal', 'usedLineHeight');
  }, `getComputedStyle(<input type=${input.type}>).lineHeight should return a used value that is no smaller than 'normal' (but should not literally be 'normal')`);
  test(() => {
    const computedLineHeight = input.computedStyleMap().get('line-height');
    assert_equals(computedLineHeight.value, 1, 'computedLineHeight.value');
    assert_equals(computedLineHeight.unit, 'px', 'computedLineHeight.unit');
  }, `<input type=${input.type}>.computedStyleMap().get('line-height') should not be affected by the used value clamping`);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “url” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 724,
        "byte_start": 698,
        "col": 4,
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/input-line-height-computed.html"
}
```
