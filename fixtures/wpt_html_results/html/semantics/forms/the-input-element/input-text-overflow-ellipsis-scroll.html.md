# html/semantics/forms/the-input-element/input-text-overflow-ellipsis-scroll.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-text-overflow-ellipsis-scroll.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<title>Input with text-overflow: ellipsis should remove ellipsis when scrolled</title>
<link rel="help" href="https://issues.chromium.org/issues/40340795">
<link rel="match" href="input-text-overflow-ellipsis-scroll-ref.html">
<style>
  input {
    width: 100px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-family: monospace;
    font-size: 20px;
  }
</style>
<input type="text" value="01234567890123456789">
<script>
  requestAnimationFrame(() => {
    const input = document.querySelector('input');
    // Scroll to end
    input.scrollLeft = input.scrollWidth;
    document.documentElement.classList.remove('reftest-wait');
  });
</script>
</html>
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
  "source_name": "html/semantics/forms/the-input-element/input-text-overflow-ellipsis-scroll.html"
}
```
