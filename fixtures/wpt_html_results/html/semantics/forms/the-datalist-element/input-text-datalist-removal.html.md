# html/semantics/forms/the-datalist-element/input-text-datalist-removal.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-datalist-element/input-text-datalist-removal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://issues.chromium.org/issues/342660638">
<link rel=match href="input-text-focused-ref.html">
<link rel=assert title="Text inputs should reset their datalist related appearance when the list attribute is removed.">

<style>
input {
  caret-color: transparent;
}
</style>

<input list=mydatalist>
<datalist id=mydatalist>
  <option>option</option>
</datalist>

<script>
const input = document.querySelector('input');
(async () => {
  input.focus();
  await new Promise(requestAnimationFrame);
  input.removeAttribute('list');
  await new Promise(requestAnimationFrame);
  input.focus();
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
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 334,
        "byte_start": 214,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/forms/the-datalist-element/input-text-datalist-removal.html"
}
```
