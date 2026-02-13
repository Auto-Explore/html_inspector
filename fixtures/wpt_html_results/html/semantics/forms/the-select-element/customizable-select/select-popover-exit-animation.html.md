# html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<meta name=fuzzy content="maxDifference=0-41;totalPixels=0-6">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=match href="select-popover-exit-animation-ref.html">
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>

<div>hover target</div>
<select>
  <option>option</option>
</select>

<style>
select,::picker(select) {
  appearance: base-select;
}
select.animate::picker(select) {
  transition: display 10000s allow-discrete, overlay 10000s allow-discrete;
}
</style>

<script>
(async () => {
  const select = document.querySelector('select');
  await test_driver.click(select);
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  select.classList.add('animate');
  await test_driver.click(select);
  select.blur();
  await (new test_driver.Actions()
    .pointerMove(1, 1, {origin: document.querySelector('div')}))
    .send();
  await new Promise(requestAnimationFrame);
  await new Promise(requestAnimationFrame);
  document.documentElement.classList.remove('reftest-wait');
})();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 460,
        "byte_start": 453,
        "col": 1,
        "line": 15
      }
    },
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
  "source_name": "html/semantics/forms/the-select-element/customizable-select/select-popover-exit-animation.html"
}
```
