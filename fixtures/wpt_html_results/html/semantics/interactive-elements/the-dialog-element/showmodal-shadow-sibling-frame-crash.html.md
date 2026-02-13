# html/semantics/interactive-elements/the-dialog-element/showmodal-shadow-sibling-frame-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/showmodal-shadow-sibling-frame-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=test-wait>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:noel@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=804047">

<template>
  <custom-dialog></custom-dialog>
</template>
<div id=shadow></div>
<iframe id=sibling></iframe>

<script>
customElements.define('custom-dialog',class extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({mode: 'open'}).innerHTML = '<dialog></dialog>';
  }
  show() {
    this.shadowRoot.querySelector('dialog').showModal();
  }
});

onload = () => {
  const template = document.querySelector('template');
  const content = document.importNode(template.content, true);
  const dialog = content.querySelector('custom-dialog');
  document.querySelector('div').appendChild(dialog);
  dialog.show();
  document.documentElement.classList.remove('test-wait');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/showmodal-shadow-sibling-frame-crash.html"
}
```
