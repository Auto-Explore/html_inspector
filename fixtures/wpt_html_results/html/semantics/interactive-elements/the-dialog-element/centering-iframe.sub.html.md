# html/semantics/interactive-elements/the-dialog-element/centering-iframe.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/centering-iframe.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>dialog element centered frame</title>
<style>
  html {
    writing-mode: {{GET[html-writing-mode]}}
  }

  #container {
    writing-mode: {{GET[container-writing-mode]}}
  }

  dialog {
    writing-mode: {{GET[dialog-writing-mode]}};
    border: none;
    padding: 0;
    max-width: initial;
    max-height: initial;
    width: {{GET[dialog-width]}};
    height: {{GET[dialog-height]}};
  }
</style>

<div id="container">
  <dialog>X</dialog>
</div>

<script>
"use strict";
document.querySelector("dialog").showModal();
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/centering-iframe.sub.html"
}
```
