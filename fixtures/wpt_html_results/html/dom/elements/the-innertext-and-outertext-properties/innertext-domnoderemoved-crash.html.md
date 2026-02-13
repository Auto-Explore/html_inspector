# html/dom/elements/the-innertext-and-outertext-properties/innertext-domnoderemoved-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/the-innertext-and-outertext-properties/innertext-domnoderemoved-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1411135">

<div id=parentelement><div id=childelement>hello world</div></div>

<script>
  let removed = false;
  childelement.addEventListener('DOMNodeRemoved', () => {
    if (!removed) {
      removed = true;
      childelement.remove();
    }
  });
  parentelement.innerText = 'hello world';
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
  "source_name": "html/dom/elements/the-innertext-and-outertext-properties/innertext-domnoderemoved-crash.html"
}
```
