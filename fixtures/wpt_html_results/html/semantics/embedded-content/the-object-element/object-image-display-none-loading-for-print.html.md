# html/semantics/embedded-content/the-object-element/object-image-display-none-loading-for-print.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-image-display-none-loading-for-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test loading of 'display: none' image for print</title>

<link rel="help" href="https://crbug.com/41477900">
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#the-object-element">
<!--
  Based on step 2 of the spec algorithm:
  "... if the element is not being rendered,
  then jump to the step below labeled fallback."
-->

<link rel="match" href="object-image-display-none-loading-for-print-ref.html">

<style>
  #target {
    display: none;
  }
</style>

<script>
  function obj_onload() {
    const p = document.createElement('p');
    p.innerHTML = `FAIL: Object image was loaded.`;
    p.style.color = 'red';
    document.body.appendChild(p);
  }
</script>

<p>
  Object image not displayed should not load.
</p>

<div>
<object
  data="/images/red.png"
  id="target"
  onload="obj_onload();"
  type="image/png"
></object>
</div>
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-image-display-none-loading-for-print.html"
}
```
