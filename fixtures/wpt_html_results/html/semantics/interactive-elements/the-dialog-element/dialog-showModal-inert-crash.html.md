# html/semantics/interactive-elements/the-dialog-element/dialog-showModal-inert-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-showModal-inert-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>

<!-- This test passes if it does not crash. -->

<iframe id="frame"></iframe>
<script>
  window.onload = () => {
    const host = document.createElement("div");
    frame.appendChild(host);
    frame.contentDocument.body.innerHTML = "<dialog></dialog>";
    document.body.offsetTop;
    const root = host.attachShadow({mode: 'open'});
    root.innerHTML = "<content>";
    const dialog = frame.contentDocument.querySelector("dialog");
    dialog.showModal();
  };
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-showModal-inert-crash.html"
}
```
