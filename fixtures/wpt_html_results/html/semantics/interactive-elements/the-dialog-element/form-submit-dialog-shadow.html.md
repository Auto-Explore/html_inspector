# html/semantics/interactive-elements/the-dialog-element/form-submit-dialog-shadow.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/form-submit-dialog-shadow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/7971">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<dialog>
  <div id=host>
    <template shadowrootmode=open>
      <form method=dialog>
        <button>close</button>
      </form>
    </template>
  </div>
</dialog>

<script>
test(() => {
  const dialog = document.querySelector('dialog');
  const host = document.getElementById('host');
  const button = host.shadowRoot.querySelector('button');

  dialog.showModal();
  button.click();
  assert_true(dialog.open);
}, '<form method=dialog> should not submit across shadow boundaries.');
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/form-submit-dialog-shadow.html"
}
```
