# html/semantics/interactive-elements/the-dialog-element/dialog-form-submission-unusual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/dialog-form-submission-unusual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test dialog form submission, unusual cases</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=log></div>

<dialog>
  <form method=dialog action="https://test:test/" target=doesnotmatter rel=noopener>
    <input type=submit formaction="https://test:test/" id=submit-1>
    <input type=submit id=submit-2>
  </form>
</dialog>

<script>
test(() => {
  const dialog = document.querySelector("dialog");
  dialog.showModal();
  assert_true(dialog.open);

  document.getElementById("submit-1").click();
  assert_false(dialog.open);
}, "A form's action and rel=noopener are ignored during submission");

test(() => {
  const dialog = document.querySelector("dialog");
  dialog.showModal();
  assert_true(dialog.open);

  document.getElementById("submit-2").click();
  assert_false(dialog.open);
}, "A form's action and rel=noopener are ignored during submission, part 2");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.form.action.invalid",
      "message": "Bad value “https://test:test/” for attribute “action” on element “form”.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 212,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.input.formaction.invalid",
      "message": "Bad value “https://test:test/” for attribute “formaction” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 362,
        "byte_start": 299,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/dialog-form-submission-unusual.html"
}
```
