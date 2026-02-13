# html/semantics/interactive-elements/the-dialog-element/modal-dialog-scroll-height.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-scroll-height.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="viewport" content="width=device-width,initial-scale=1">
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=author href="mailto:skobes@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/interactive-elements.html#the-dialog-element">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=403136">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<style>
body {
  margin: 0;
}
.spacer {
  height: 500px;
}
dialog {
  border: 0;
  margin: 0;
  padding: 1px;
}
</style>
<div class="spacer"></div>
<dialog>
  <div class="spacer"></div>
</dialog>

<script>
test(() => {
  document.querySelector('dialog').showModal();
  assert_equals(document.scrollingElement.scrollHeight, window.innerHeight);
}, 'dialogs should be centered before computing overflow.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/modal-dialog-scroll-height.html"
}
```
