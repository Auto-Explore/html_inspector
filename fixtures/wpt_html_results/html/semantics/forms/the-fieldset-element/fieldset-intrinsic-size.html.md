# html/semantics/forms/the-fieldset-element/fieldset-intrinsic-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/fieldset-intrinsic-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Fieldset with intrinsic size</title>
<link rel="author" title="Oriol Brufau" href="mailto:obrufau@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-fieldset-element">
<link rel="help" href="https://drafts.csswg.org/css-sizing-3/#intrinsic">
<meta name="assert" content="A fieldset with an intrinsic size should be as big as required by the contents.">
<style>
fieldset {
  height: min-content;
  padding: 7px;
  border: 3px solid cyan;
}
fieldset > div {
  border: 3px solid orange;
}
.auto {
  height: auto;
}
.min-content {
  height: min-content;
}
.max-content {
  height: max-content;
}
.content-box {
  box-sizing: content-box;
}
.border-box {
  box-sizing: border-box;
}
</style>

<div id="log"></div>

<fieldset class="auto content-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>
<fieldset class="auto border-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>
<fieldset class="min-content content-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>
<fieldset class="min-content border-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>
<fieldset class="max-content content-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>
<fieldset class="max-content border-box">
  <legend>Legend</legend>
  <div>Contents</div>
</fieldset>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
for (let fieldset of document.querySelectorAll("fieldset")) {
  test(function() {
    const fieldsetRect = fieldset.getBoundingClientRect();
    const contentsRect = fieldset.querySelector("div").getBoundingClientRect();
    const fieldsetOuterEnd = fieldsetRect.y + fieldsetRect.height;
    const fieldsetInnerEnd = fieldsetOuterEnd - 10;
    const contentsOuterEnd = contentsRect.y + contentsRect.height;
    assert_equals(fieldsetInnerEnd, contentsOuterEnd);
  }, fieldset.className);
}
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
  "source_name": "html/semantics/forms/the-fieldset-element/fieldset-intrinsic-size.html"
}
```
