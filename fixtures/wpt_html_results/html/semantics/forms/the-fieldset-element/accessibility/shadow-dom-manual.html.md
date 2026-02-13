# html/semantics/forms/the-fieldset-element/accessibility/shadow-dom-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/accessibility/shadow-dom-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset accessibility test: shadow DOM</title>
<link rel=help href=https://w3c.github.io/html-aam/#fieldset-element-accessible-name-computation>
<template id="my-fieldset">
  <fieldset id=fieldset>
    <slot name="my-text"></slot>
    <input>
  </fieldset>
</template>

<my-fieldset>
  <legend slot="my-text">Foo</legend>
</my-fieldset>

<p>Expected accessible name for id=fieldset: ""

<script>
customElements.define('my-fieldset',
  class extends HTMLElement {
    constructor() {
      super();

      const template = document.getElementById('my-fieldset');
      const templateContent = template.content;

      this.attachShadow({mode: 'open'}).appendChild(
        templateContent.cloneNode(true)
      );
    }
  }
);
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
  "source_name": "html/semantics/forms/the-fieldset-element/accessibility/shadow-dom-manual.html"
}
```
