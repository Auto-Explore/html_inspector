# html/semantics/interactive-elements/the-dialog-element/showmodal-in-shadow-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/showmodal-in-shadow-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:futhark@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=850664">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=851384">

<div id="dialog">
  <div id="item"></div>
</div>
<script>
const itemRoot = item.attachShadow({mode: 'open'});
const dialogRoot = dialog.attachShadow({mode: 'open'});
dialogRoot.innerHTML = '<dialog><slot></slot></dialog>';
dialog.offsetTop;
dialogRoot.firstChild.showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/showmodal-in-shadow-crash.html"
}
```
