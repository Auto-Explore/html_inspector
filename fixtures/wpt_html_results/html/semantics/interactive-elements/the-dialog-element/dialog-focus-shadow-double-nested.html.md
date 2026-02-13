# html/semantics/interactive-elements/the-dialog-element/dialog-focus-shadow-double-nested.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-focus-shadow-double-nested.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>dialog focusing delegation: with two nested shadow trees</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>

<dialog>
  <template class="turn-into-shadow-tree delegates-focus">
    <button disabled>Non-focusable</button>
    <template class="turn-into-shadow-tree delegates-focus">
      <button tabindex="-1">Focusable</button>
      <button tabindex="-1" autofocus>Focusable</button>
      <button tabindex="-1">Focusable</button>
    </template>
    <button tabindex="-1">Focusable</button>
  </template>
  <button tabindex="-1">Focusable</button>
</dialog>

<script>
function turnIntoShadowTree(template) {
  for (const subTemplate of template.content.querySelectorAll(".turn-into-shadow-tree")) {
    turnIntoShadowTree(subTemplate);
  }

  const div = document.createElement("div");
  div.attachShadow({ mode: "open", delegatesFocus: template.classList.contains("delegates-focus") });
  div.shadowRoot.append(template.content);
  template.replaceWith(div);
}

for (const template of document.querySelectorAll(".turn-into-shadow-tree")) {
  turnIntoShadowTree(template);
}

for (const method of ["show", "showModal"]) {
  test(t => {
    const dialog = document.querySelector("dialog");
    dialog[method]();
    t.add_cleanup(() => dialog.close());

    const shadowHostOuter = dialog.querySelector("div");
    assert_equals(document.activeElement, shadowHostOuter, "document.activeElement");

    const shadowHostInner = shadowHostOuter.shadowRoot.querySelector("div");
    assert_equals(shadowHostOuter.shadowRoot.activeElement, shadowHostInner, "shadowHostOuter.shadowRoot.activeElement");

    const button = shadowHostInner.shadowRoot.querySelector("[autofocus]");
    assert_equals(shadowHostInner.shadowRoot.activeElement, button, "shadowHostInner.shadowRoot.activeElement");
  }, `${method}()`);
}
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-focus-shadow-double-nested.html"
}
```
