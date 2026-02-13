# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol-column-height.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol-column-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset multicol with auto count, non-auto width</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-fieldset-and-legend-elements">
<link rel="help" href="https://drafts.csswg.org/css-multicol-2/#ch">
<link rel="help" href="https://drafts.csswg.org/css-multicol-2/#cwr">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
 #test { margin: 0; padding: 0; border: none }
 #test, #ref { columns:3; column-height:1.5em; column-wrap:wrap; gap:20px; }
 p { margin: 0 }
</style>
<fieldset id="test">
  <p>1</p>
  <p>2</p>
  <p>3</p>
  <p>4</p>
  <p>5</p>
</fieldset>
<div id="ref">
  <p>1</p>
  <p>2</p>
  <p>3</p>
  <p>4</p>
  <p>5</p>
</div>
<script>
  test(() => {
    assert_equals(getComputedStyle(document.getElementById('test')).height,
                  getComputedStyle(document.getElementById('ref')).height);
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-multicol-column-height.html"
}
```
